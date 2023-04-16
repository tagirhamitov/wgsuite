use base64::{engine::general_purpose, Engine};
use crypto_box::{rand_core::OsRng, SecretKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub private: String,
    pub public: String,
}

impl KeyPair {
    pub fn generate() -> Self {
        let private = SecretKey::generate(&mut OsRng);
        let public = private.public_key();

        let private = encode_base64(private.as_bytes());
        let public = encode_base64(public.as_bytes());

        Self { private, public }
    }
}

fn encode_base64(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}
