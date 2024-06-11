use std::fs;

use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret_file = std::env::var("JWT_SECRET_FILE").expect("JWT_SECRET_FILE must be set");
    let secret = fs::read_to_string(secret_file).expect("Failed to read JWT secret file");
    Keys::new(secret.as_bytes())
});