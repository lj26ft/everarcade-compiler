use serde::Serialize;

use crate::hashing::sha256;

pub fn canonical_bytes<T: Serialize>(value: &T) -> anyhow::Result<Vec<u8>> {
    Ok(serde_json::to_vec(value)?)
}

pub fn canonical_hash<T: Serialize>(value: &T) -> anyhow::Result<[u8; 32]> {
    Ok(sha256(&canonical_bytes(value)?))
}
