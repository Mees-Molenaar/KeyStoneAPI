use axum::{http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use rsa::traits::PublicKeyParts;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

use crate::rsa::RsaKeyPair;

#[derive(Debug, Serialize, Deserialize)]
struct Jwk {
    kty: String,
    alg: String,
    use_: String,
    kid: String,
    n: String,
    e: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwks {
    keys: Vec<Jwk>,
}

pub async fn get_jwks(
    Extension(rsa_keypair): Extension<RsaKeyPair>
) -> Result<Json<Jwks>, StatusCode> {
    Ok(Json(Jwks {
        keys: vec![Jwk {
            kty: "RSA".to_string(),
            alg: "RS256".to_string(),
            // Same way of encoding as the jsonwebtoken serialization module
            e: URL_SAFE_NO_PAD.encode(rsa_keypair.public_key.e().to_bytes_be()),
            n: URL_SAFE_NO_PAD.encode(rsa_keypair.public_key.n().to_bytes_be()),
            use_: "sig".to_string(),
            kid: "1".to_string(),
        }],
    }))
}