use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn canonical_json<T: Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_string(value).map_err(|e| format!("serialization failure: {e}"))
}

pub fn stable_hash(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

pub fn hash_serialized<T: Serialize>(value: &T) -> Result<String, String> {
    canonical_json(value).map(|j| stable_hash(&j))
}
