use keystone_auth::{app, database};

#[tokio::main]

async fn main() {
    let pool = database::setup_database()
        .await
        .expect("Failed to setup database");

    database::create_user(&pool).await.expect("Failed to add user to database");

    let app = app::setup_app(pool).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
