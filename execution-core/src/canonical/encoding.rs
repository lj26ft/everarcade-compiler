use serde::{de::DeserializeOwned, Serialize};

use super::errors::CanonicalError;

pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalError> {
    bincode::serialize(value).map_err(|e| CanonicalError::Encode(e.to_string()))
}

pub fn canonical_decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, CanonicalError> {
    bincode::deserialize(bytes).map_err(|e| CanonicalError::Decode(e.to_string()))
}
