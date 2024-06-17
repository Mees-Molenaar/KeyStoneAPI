use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::auth::Claims;

#[derive(Deserialize, Debug, Serialize)]
pub struct NewDocument {
    title: String,
    content: String,
}
#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Document {
    id: i32,
    user_id: String,
    title: String,
    content: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    is_synced: bool,
    last_synced_at: Option<chrono::NaiveDateTime>,
}

pub async fn create_document(
    State(pool): State<PgPool>,
    claims: Claims,
    Json(payload): Json<NewDocument>,
) -> Result<(StatusCode, Json<Document>), StatusCode> {
    println!("Received document: {:?}", payload);

    let result: Document = sqlx::query_as(
        r#"
        INSERT INTO document (user_id, title, content)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, title, content, created_at, updated_at, is_synced, last_synced_at
        "#,
    )
    .bind(&claims.sub)
    .bind(&payload.title)
    .bind(&payload.content)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to insert document: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    println!("Document inserted successfully: {:?}", result);

    Ok((StatusCode::CREATED, Json(result)))
}

pub async fn get_documents(
    State(pool): State<PgPool>,
    claims: Claims
) -> Result<(StatusCode, Json<Vec<Document>>), StatusCode> {
    let documents: Vec<Document> = sqlx::query_as(
        r#"
        SELECT id, user_id, title, content, created_at, updated_at, is_synced, last_synced_at
        FROM document
        WHERE user_id = $1
        "#,
    )
    .bind(claims.sub)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to get documents: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, Json(documents)))
}

pub async fn get_document(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    claims: Claims
) -> Result<(StatusCode, Json<Document>), StatusCode> {
    let document: Document = sqlx::query_as(
        r#"
            SELECT id, user_id, title, content, created_at, updated_at, is_synced, last_synced_at
            FROM document
            WHERE id = $1
            AND user_id = $2
            "#,
    )
    .bind(&id)
    .bind(&claims.sub)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to get document: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, Json(document)))
}
