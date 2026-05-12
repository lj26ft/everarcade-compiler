use serde::Serialize;

pub fn encode<T: Serialize>(value: &T) -> Vec<u8> { super::canonical::canonical_bytes(value) }
