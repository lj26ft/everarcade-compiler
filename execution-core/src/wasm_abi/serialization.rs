use super::errors::WasmAbiError;
use bincode::Options;
use serde::{de::DeserializeOwned, Serialize};

pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, WasmAbiError> {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .serialize(value)
        .map_err(|e| WasmAbiError::Serialization(e.to_string()))
}

pub fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, WasmAbiError> {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize(bytes)
        .map_err(|e| WasmAbiError::Serialization(e.to_string()))
}
