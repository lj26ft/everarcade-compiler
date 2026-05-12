use serde::Serialize;
pub fn canonical_hash<T: Serialize>(value: &T) -> String { crate::hashing::hash_bytes(&bincode::serialize(value).expect("serialize")) }
