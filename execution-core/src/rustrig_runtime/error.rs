use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RustrigRuntimeError {
    #[error("unknown rustrig: {0}")]
    UnknownRustrig(String),
    #[error("ABI-incompatible rustrig {rustrig_id}: {reason}")]
    AbiIncompatible { rustrig_id: String, reason: String },
    #[error("invalid pipeline: {0}")]
    InvalidPipeline(String),
    #[error("authority mutation rejected: {0}")]
    AuthorityMutationRejected(String),
}

pub type Result<T> = std::result::Result<T, RustrigRuntimeError>;
