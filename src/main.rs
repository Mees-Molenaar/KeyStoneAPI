use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use keystone_api::database;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Debug, Serialize)]
struct NewDocument {
    user_id: i32,
    title: String,
    content: String,
}
#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
struct Document {
    id: i32,
    user_id: i32,
    title: String,
    content: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    is_synced: bool,
    last_synced_at: Option<chrono::NaiveDateTime>,
}

#[tokio::main]
async fn main() {
    let pool = database::setup_database().await.expect("Failed to setup database");

    // build our application with a route
    let app = Router::new()
        .route("/document", post(create_document).get(get_documents))
        .route("/document/:id", get(get_document))
        .with_state(pool);

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn create_document(
    State(pool): State<PgPool>,
    Json(payload): Json<NewDocument>,
) -> Result<(StatusCode, Json<Document>), StatusCode> {
    // Handle the payload
    println!("Received document: {:?}", payload);

    // Insert the document into the database
    let result: Document = sqlx::query_as(
        r#"
        INSERT INTO document (user_id, title, content)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, title, content, created_at, updated_at, is_synced, last_synced_at
        "#,
    )
    .bind(&payload.user_id)
    .bind(&payload.title)
    .bind(&payload.content)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to insert document: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    println!("Document inserted successfully: {:?}", result);

    // Respond with a status code
    Ok((StatusCode::CREATED, Json(result)))
}

async fn get_documents(
    State(pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<Document>>), StatusCode> {
    let documents: Vec<Document> = sqlx::query_as(
        r#"
        SELECT id, user_id, title, content, created_at, updated_at, is_synced, last_synced_at
        FROM document
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to get documents: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Respond with a status code
    Ok((StatusCode::OK, Json(documents)))
}

async fn get_document(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Document>), StatusCode> {
    let document: Document = sqlx::query_as(
        r#"
            SELECT id, user_id, title, content, created_at, updated_at, is_synced, last_synced_at
            FROM document
            WHERE id = $1
            "#,
    )
    .bind(&id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to get document: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Respond with a status code
    Ok((StatusCode::OK, Json(document)))
}
