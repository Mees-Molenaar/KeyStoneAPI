use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{auth::{AuthError, Claims}, rsa::{generate_jwt, RsaKeyPair}};

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    id: i32,
    username: String,
    password_hash: String,
}

pub async fn authorize(
    State(pool): State<PgPool>,
    Extension(rsa_keypair): Extension<RsaKeyPair>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let user: User = sqlx::query_as(
        r#"
        SELECT * FROM users
        WHERE username = $1
        LIMIT 1
        "#,
    )
    .bind(&payload.client_id)
    .fetch_one(&pool)
    .await
    .map_err(|_| AuthError::WrongCredentials)?;

    let parsed_hash = PasswordHash::new(&user.password_hash).expect("Failed to parse hash");

    if !Argon2::default()
        .verify_password(payload.client_secret.as_bytes(), &parsed_hash)
        .is_ok()
    {
        return Err(AuthError::WrongCredentials);
    }

    let claims = Claims {
        sub: user.id.to_string(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = generate_jwt(&claims, &rsa_keypair.private_key);

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}
