use axum::{
    routing::post,
    Router,
};
use sqlx::PgPool;

use crate::routes::authorize::authorize;

pub async fn setup_app(pool: PgPool) -> Router {
    Router::new()
        .route("/authorize", post(authorize))
        .with_state(pool)
}
