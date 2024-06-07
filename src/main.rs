use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
    extract::Path
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
struct NewDocument {
    user_id: i32,
    content: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct Document {
    id: i32,
    user_id: i32,
    content: String,
    created_at: chrono::NaiveDateTime,
    is_synced: bool,
    last_synced_at: Option<chrono::NaiveDateTime>,
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/document", 
        post(create_document)
        .get(get_documents))
        .route("/document/:id", get(get_document));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn create_document(
    Json(payload): Json<NewDocument>,
) -> Result<(StatusCode, Json<NewDocument>), StatusCode> {
    // Handle the payload
    println!("Received user: {:?}", payload);

    // Respond with a status code
    Ok((StatusCode::CREATED, Json(payload)))
}

async fn get_documents(
) -> Result<(StatusCode, Json<Vec<Document>>), StatusCode> {    
    let documents = vec![
        Document {
            id: 1,
            user_id: 1,
            content: "Hello, World!".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            is_synced: true,
            last_synced_at: Some(chrono::Utc::now().naive_utc()),
        },
        Document {
            id: 2,
            user_id: 1,
            content: "Hello, World!".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            is_synced: true,
            last_synced_at: Some(chrono::Utc::now().naive_utc()),
        },
    ];
    // Respond with a status code
    Ok((StatusCode::OK, Json(documents)))
}

async fn get_document(
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Document>), StatusCode> {    
    let document = 
        Document {
            id: 1,
            user_id: 1,
            content: "Hello, World!".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            is_synced: true,
            last_synced_at: Some(chrono::Utc::now().naive_utc()),
        };

    // Respond with a status code
    Ok((StatusCode::OK, Json(document)))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
