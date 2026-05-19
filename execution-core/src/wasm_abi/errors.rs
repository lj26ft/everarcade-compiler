use thiserror::Error;

#[derive(Debug, Error)]
pub enum WasmAbiError {
    #[error("ABI version mismatch: expected {expected}, got {actual}")]
    AbiVersionMismatch { expected: u32, actual: u32 },
    #[error("invalid result handle")]
    InvalidResultHandle,
    #[error("missing export: {0}")]
    MissingExport(&'static str),
    #[error("guest memory bounds check failed")]
    MemoryBounds,
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("wasm runtime error: {0}")]
    Runtime(String),
}
