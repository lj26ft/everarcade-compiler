use serde::Serialize;

pub fn canonical_encode<T: Serialize>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).expect("canonical JSON encoding should not fail")
}
