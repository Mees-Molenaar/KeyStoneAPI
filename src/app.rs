use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::routes::document::{create_document, get_document, get_documents};

pub async fn setup_app(pool: PgPool) -> Router {
    Router::new()
        .route("/document", post(create_document).get(get_documents))
        .route("/document/:id", get(get_document))
        .with_state(pool)
}
