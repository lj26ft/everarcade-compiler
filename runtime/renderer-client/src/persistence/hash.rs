use serde::Serialize;

use super::crypto;

pub fn canonical_json<T: Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_string(value).map_err(|e| format!("serialization failure: {e}"))
}

pub fn stable_hash(value: &str) -> String {
    crypto::hash_projection(&value).expect("hash string").0
}

pub fn hash_serialized<T: Serialize>(value: &T) -> Result<String, String> {
    crypto::hash_projection(value).map(|h| h.0)
}
