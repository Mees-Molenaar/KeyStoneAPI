use std::{env, fs};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn setup_database() -> Result<PgPool, sqlx::Error> {
    let db_connect_str_file = env::var("DATABASE_CONNECTION_STRING_FILE")
        .expect("DATABASE_CONNECTION_STRING_FILE must be set");
    let db_connection_str = std::fs::read_to_string(db_connect_str_file)
        .expect("DATABASE_CONNECTION_STRING_FILE must be a valid file path");

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

pub async fn create_user(pool: &PgPool) -> Result<(), sqlx::Error> {
    let username = env::var("USERNAME").expect("USERNAME must be set");
    let user_password_file = env::var("USER_PASSWORD_FILE").expect("USER_PASSWORD_FILE must be set");
    let user_password = fs::read_to_string(user_password_file).expect("Failed to read password file");

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(user_password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    sqlx::query(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(username)
    .bind(password_hash)
    .execute(pool)
    .await?;

    Ok(())
}
