use crate::repository::db::Database;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, http::StatusCode, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
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

static DUMMY_HASH: OnceLock<String> = OnceLock::new();

fn get_dummy_hash() -> &'static str {
    DUMMY_HASH.get_or_init(|| {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password("dummy_password".as_bytes(), &salt)
            .unwrap()
            .to_string()
    })
}

pub async fn signup(
    State(db): State<Database>,
    Json(payload): Json<AuthPayload>,
) -> Result<StatusCode, StatusCode> {
    // 0. Validate Input
    if payload.validate().is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 1. Check if user already exists
    // SECURITY: Prevent duplicate users (DoS/Account Takeover risk)
    let check_sql = "SELECT id FROM user WHERE email = $email LIMIT 1";
    let mut check_response = db
        .client
        .query(check_sql)
        .bind(("email", payload.email.clone()))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let existing: Vec<serde_json::Value> = check_response
        .take(0)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !existing.is_empty() {
        return Err(StatusCode::CONFLICT);
    }

    // 2. Hash Password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2
        .hash_password(payload.password.as_bytes(), &salt) {
            Ok(hash) => hash.to_string(),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

    // 3. Create User in DB (Simplified for MVP)
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
    let sql = "SELECT * FROM user WHERE email = $email LIMIT 1";
    let mut response = db
        .client
        .query(sql)
        .bind(("email", payload.email.clone()))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users: Vec<serde_json::Value> = response
        .take(0)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = users.into_iter().next();

    // SECURITY: Prevent Timing Attacks (Username Enumeration)
    // Always perform password verification, even if the user is not found.
    // This ensures that the response time is roughly the same for valid and invalid emails.

    let (password_valid, _user_found) = if let Some(user) = user {
        let stored_hash = user.get("password").and_then(|v| v.as_str()).unwrap_or("");

        // If stored hash is invalid (e.g. empty or bad format), we treat it as auth failure
        // but still try to verify against dummy to consume time.
        if let Ok(parsed_hash) = PasswordHash::new(stored_hash) {
            (
                Argon2::default()
                    .verify_password(payload.password.as_bytes(), &parsed_hash)
                    .is_ok(),
                true,
            )
        } else {
            // Invalid stored hash format - use dummy to consume time
            let parsed_dummy = PasswordHash::new(get_dummy_hash()).unwrap();
            let _ = Argon2::default().verify_password(payload.password.as_bytes(), &parsed_dummy);
            (false, true)
        }
    } else {
        // User not found - verify against dummy hash to consume time
        let parsed_dummy = PasswordHash::new(get_dummy_hash()).unwrap();
        let _ = Argon2::default().verify_password(payload.password.as_bytes(), &parsed_dummy);
        (false, false)
    };

    if password_valid {
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

    #[test]
    fn test_dummy_hash_generation() {
        // Ensure the dummy hash is generated successfully
        let hash = get_dummy_hash();
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2"));
    }
}
