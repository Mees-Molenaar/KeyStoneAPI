use jsonwebtoken::{
    decode, decode_header, errors::ErrorKind as JwtError, Algorithm, DecodingKey, Validation,
};
use serde::{Deserialize, Serialize};

use crate::auth::Claims;

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

pub async fn fetch_jwks(jwks_url: &str) -> Result<Jwks, reqwest::Error> {
    let client = reqwest::ClientBuilder::new().use_rustls_tls().build()?;
    
    client.get(jwks_url).send().await?.json::<Jwks>().await
}

pub async fn verify_jwt(token: &str, jwks: &Jwks) -> Result<Claims, jsonwebtoken::errors::Error> {
    let header = decode_header(token)?;

    let kid = header.kid.ok_or(JwtError::InvalidKeyFormat)?;

    let jwk = jwks
        .keys
        .iter()
        .find(|key| key.kid == kid)
        .ok_or(JwtError::InvalidKeyFormat)?;

    let decoding_key =
        DecodingKey::from_rsa_components(&jwk.n, &jwk.e).map_err(|_| JwtError::InvalidKeyFormat)?;
    let validation = Validation::new(Algorithm::RS256);
    let token_data = decode::<Claims>(token, &decoding_key, &validation);

    Ok(token_data?.claims)
}
