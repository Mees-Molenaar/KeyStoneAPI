use keystone_api::{app, database};

#[tokio::main]
async fn main() {
    let pool = database::setup_database()
        .await
        .expect("Failed to setup database");

    let app = app::setup_app(pool).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
