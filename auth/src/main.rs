use keystone_auth::{app, database, rsa};

#[tokio::main]

async fn main() {

    let rsa_keypair = rsa::generate_rsa_keys();

    let pool = database::setup_database()
        .await
        .expect("Failed to setup database");

    database::create_user(&pool).await.expect("Failed to add user to database");

    let app = app::setup_app(pool, rsa_keypair).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
