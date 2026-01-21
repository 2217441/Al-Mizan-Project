use crate::repository::db::Database;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, http::StatusCode, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AuthPayload {
    #[validate(email, length(max = 255))]
    email: String,
    #[validate(length(min = 8, max = 128))]
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn signup(
    State(db): State<Database>,
    Json(payload): Json<AuthPayload>,
) -> Result<StatusCode, StatusCode> {
    // 0. Validate Input
    if payload.validate().is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 1. Hash Password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR) {
            Ok(hash) => hash.to_string(),
            Err(e) => return Err(e),
        };

    // 2. Create User in DB (Simplified for MVP)
    let sql = "CREATE user SET email = $email, password = $password, role = 'student'";
    let created = db
        .client
        .query(sql)
        .bind(("email", payload.email))
        .bind(("password", password_hash))
        .await
        .map(|mut r| r.take::<Option<serde_json::Value>>(0));

    if created.is_err() || created.as_ref().unwrap().is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::CREATED)
}

pub async fn signin(
    State(db): State<Database>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // 1. Fetch User
    let sql = "SELECT * FROM user WHERE email = $email";
    let mut response = db
        .client
        .query(sql)
        .bind(("email", payload.email.clone()))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user: Option<serde_json::Value> = response
        .take(0)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(user) = user {
        // 2. Verify Password
        let stored_hash = user.get("password").and_then(|v| v.as_str()).unwrap_or("");
        let parsed_hash = PasswordHash::new(stored_hash).map_err(|_| StatusCode::UNAUTHORIZED)?;

        if Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            // 3. Generate JWT
            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(24))
                .expect("valid timestamp")
                .timestamp();

            let claims = Claims {
                sub: payload.email,
                exp: usize::try_from(expiration).unwrap_or(0),
            };

            // SECURITY: JWT_SECRET must be set in production
            let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
                assert!(
                    std::env::var("RUST_ENV").unwrap_or_default() != "production",
                    "JWT_SECRET must be set in production environment"
                );
                tracing::warn!("Using insecure dev JWT secret - DO NOT USE IN PRODUCTION");
                "INSECURE_DEV_SECRET_CHANGE_ME_32CH".to_string()
            });

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(jwt_secret.as_bytes()),
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            return Ok(Json(AuthResponse { token }));
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_payload_validation() {
        // Valid case
        let valid = AuthPayload {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        assert!(valid.validate().is_ok());

        // Invalid email format
        let invalid_email = AuthPayload {
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
        };
        assert!(invalid_email.validate().is_err());

        // Password too short
        let short_password = AuthPayload {
            email: "test@example.com".to_string(),
            password: "short".to_string(),
        };
        assert!(short_password.validate().is_err());

        // DoS Prevention Checks

        // Email too long (> 255 chars)
        let long_email = AuthPayload {
            email: format!("{}@example.com", "a".repeat(250)), // Total > 255
            password: "password123".to_string(),
        };
        assert!(long_email.validate().is_err());

        // Password too long (> 128 chars) - Argon2 DoS vector
        let long_password = AuthPayload {
            email: "test@example.com".to_string(),
            password: "a".repeat(129),
        };
        assert!(long_password.validate().is_err());
    }
}
