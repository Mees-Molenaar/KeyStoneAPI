use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand::rngs::OsRng;
use rsa::{pkcs1::EncodeRsaPrivateKey, RsaPrivateKey, RsaPublicKey};

use crate::auth::Claims;

#[derive(Clone, Debug)]
pub struct RsaKeyPair {
    pub private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey,
    pub kid: String,
}

impl RsaKeyPair {
    fn new(private_key: RsaPrivateKey, public_key: RsaPublicKey) -> Self {
        Self {
            private_key,
            public_key,
            kid: "1".to_string(), //Since there is only one key pair, we can hardcode the key ID
        }
    }
}

pub fn generate_rsa_keys() -> RsaKeyPair {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    RsaKeyPair::new(private_key, public_key)
}

pub fn generate_jwt(claims: &Claims, private_key: &RsaPrivateKey, kid: &str) -> String {
    let der_key = private_key
        .to_pkcs1_der()
        .expect("Failed to convert key to DER format");

    let encoding_key = EncodingKey::from_rsa_der(der_key.as_bytes());
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(kid.to_string());

    encode(&header, claims, &encoding_key).expect("Failed to encode token")
}
