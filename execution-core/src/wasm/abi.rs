use serde::{Deserialize, Serialize};

use super::serialization::canonical_bytes;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalAbiEnvelope {
    pub version: u32,
    pub payload: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbiExecutionRequest {
    pub contract_id: String,
    pub input: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbiExecutionResponse {
    pub output: Vec<u8>,
    pub events: Vec<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbiMutationSet {
    pub mutations: Vec<(String, Vec<u8>)>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbiStateEnvelope {
    pub state_root: String,
    pub state: Vec<(String, Vec<u8>)>,
}

pub fn encode<T: Serialize>(value: &T) -> anyhow::Result<Vec<u8>> {
    canonical_bytes(value)
}
pub fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> anyhow::Result<T> {
    Ok(serde_json::from_slice(bytes)?)
}
pub fn decode_handle(raw: u64) -> (u32, u32) {
    ((raw >> 32) as u32, (raw & 0xffff_ffff) as u32)
}
