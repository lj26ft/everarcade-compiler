use bincode::Options;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HostCall {
    ReadInput { seq: u64 },
    WriteOutput { seq: u64, bytes_len: u32 },
    EmitStateDiff { seq: u64, bytes_len: u32 },
    EmitLog { seq: u64, bytes_len: u32 },
    Abort { seq: u64, code: u32 },
}

pub fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, bincode::Error> {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .serialize(value)
}

pub fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, bincode::Error> {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize(bytes)
}

#[inline]
pub fn encode_handle(ptr: u32, len: u32) -> u64 {
    ((ptr as u64) << 32) | (len as u64)
}

#[inline]
pub fn decode_handle(handle: u64) -> (u32, u32) {
    ((handle >> 32) as u32, handle as u32)
}
