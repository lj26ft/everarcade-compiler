use thiserror::Error;

#[derive(Debug, Error)]
pub enum WasmRuntimeError {
    #[error("wasm module missing exported memory")]
    MissingMemory,
    #[error("wasm module missing alloc export")]
    MissingAlloc,
    #[error("wasm module missing everarcade_execute export")]
    MissingEntrypoint,
    #[error("wasm module returned invalid result handle")]
    InvalidResultHandle,
    #[error("failed ABI serialization: {0}")]
    AbiSerialization(String),
    #[error("failed ABI deserialization: {0}")]
    AbiDeserialization(String),
}
