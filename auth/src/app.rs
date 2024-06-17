use axum::{
    routing::{post, get}, Extension, Router
};
use sqlx::PgPool;

use crate::{routes::{authorize::authorize, jwks::get_jwks}, rsa::RsaKeyPair};

pub async fn setup_app(pool: PgPool, rsa_keypair: RsaKeyPair) -> Router {
    Router::new()
        .route("/authorize", post(authorize))
        .route("/jwks", get(get_jwks))
        .with_state(pool)
        .layer(Extension(rsa_keypair))
}
