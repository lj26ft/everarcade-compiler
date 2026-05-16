use thiserror::Error;

#[derive(Debug, Error)]
pub enum CanonicalError {
    #[error("canonical encode failed: {0}")]
    Encode(String),
    #[error("canonical decode failed: {0}")]
    Decode(String),
    #[error("io failed: {0}")]
    Io(#[from] std::io::Error),
}
