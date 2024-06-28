use axum::{
    routing::{get, post},
    Json, Router,
};
use sqlx::PgPool;
use utoipa::{openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme}, Modify, OpenApi};
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use super::routes::document::{create_document, get_document, get_documents, DocumentApi};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    nest(
        (path ="/document", api=DocumentApi, tags=["Document"]),
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Access Token",
                SecurityScheme::Http(
                    HttpBuilder::new().scheme(HttpAuthScheme::Bearer).bearer_format("JWT").build()
                ))
        }
    }
}

pub async fn setup_app(pool: PgPool) -> Router {
    Router::new()
        .route("/document", post(create_document).get(get_documents))
        .route("/document/:id", get(get_document))
        .route("/openapi", get(get_openapi_spec))
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .with_state(pool)
}

async fn get_openapi_spec() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
