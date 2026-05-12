use serde::Serialize;

pub fn canonical_bytes<T: Serialize>(value: &T) -> Vec<u8> {
    bincode::serialize(value).expect("canonical serialization should succeed")
}
