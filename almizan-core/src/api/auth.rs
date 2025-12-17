use crate::repository::db::Database;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, http::StatusCode, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthPayload {
    email: String,
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
    // 1. Hash Password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    // 2. Create User in DB (Simplified for MVP)
    let sql = "CREATE user SET email = $email, password = $password, role = 'user'";
    let _created: Option<serde_json::Value> = db
        .client
        .query(sql)
        .bind(("email", payload.email))
        .bind(("password", password_hash))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .take(0)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
                exp: expiration as usize,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret("secret".as_ref()),
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            return Ok(Json(AuthResponse { token }));
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
