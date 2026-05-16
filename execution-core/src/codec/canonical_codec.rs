use serde::{de::DeserializeOwned, Serialize};

pub fn canonical_encode<T: Serialize>(value: &T) -> Vec<u8> {
    crate::canonical::encoding::canonical_encode(value).expect("canonical encoding should not fail")
}

pub fn canonical_decode<T: DeserializeOwned>(bytes: &[u8]) -> T {
    crate::canonical::encoding::canonical_decode(bytes).expect("canonical decoding should not fail")
}
