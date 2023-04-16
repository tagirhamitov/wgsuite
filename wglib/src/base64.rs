use base64::{engine::general_purpose, Engine};

pub fn decode_base64(s: &str) -> anyhow::Result<Vec<u8>> {
    Ok(general_purpose::STANDARD.decode(s)?)
}
