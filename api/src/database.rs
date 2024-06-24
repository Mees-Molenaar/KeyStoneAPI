use std::env;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn setup_database() -> Result<PgPool, sqlx::Error> {
    let db_connection_str = env::var("DATABASE_CONNECTION_STRING")
        .expect("DATABASE_CONNECTION_STRING must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate");

    Ok(pool)
}