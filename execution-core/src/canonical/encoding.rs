use serde::{de::DeserializeOwned, Serialize};

use super::errors::CanonicalError;

pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalError> {
    bincode::serde::encode_to_vec(value, bincode::config::standard().with_little_endian())
        .map_err(|e| CanonicalError::Encode(e.to_string()))
}

pub fn canonical_decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, CanonicalError> {
    let (decoded, _): (T, usize) =
        bincode::serde::decode_from_slice(bytes, bincode::config::standard().with_little_endian())
            .map_err(|e| CanonicalError::Decode(e.to_string()))?;
    Ok(decoded)
}
