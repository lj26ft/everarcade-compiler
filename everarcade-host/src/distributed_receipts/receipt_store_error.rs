use std::{fmt, io};

#[derive(Debug)]
pub enum ReceiptStoreError {
    Io(io::Error),
    Codec(bincode::Error),
    Serde(serde_json::Error),
    Validation(&'static str),
}

impl fmt::Display for ReceiptStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {e}"),
            Self::Codec(e) => write!(f, "codec: {e}"),
            Self::Serde(e) => write!(f, "serde: {e}"),
            Self::Validation(e) => write!(f, "validation: {e}"),
        }
    }
}

impl std::error::Error for ReceiptStoreError {}
impl From<io::Error> for ReceiptStoreError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<bincode::Error> for ReceiptStoreError {
    fn from(value: bincode::Error) -> Self {
        Self::Codec(value)
    }
}
impl From<serde_json::Error> for ReceiptStoreError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
