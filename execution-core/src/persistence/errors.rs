use std::{fmt::Display, io};

#[derive(Debug)]
pub enum PersistenceError {
    Io(io::Error),
    Encode(bincode::Error),
    Decode(bincode::Error),
    CheckpointRootMismatch { expected: [u8; 32], actual: [u8; 32] },
    PackageRootMismatch { expected: [u8; 32], actual: [u8; 32] },
}

impl Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::Encode(e) => write!(f, "encode error: {e}"),
            Self::Decode(e) => write!(f, "decode error: {e}"),
            Self::CheckpointRootMismatch { expected, actual } => write!(f, "checkpoint root mismatch expected={} actual={}", hex::encode(expected), hex::encode(actual)),
            Self::PackageRootMismatch { expected, actual } => write!(f, "package root mismatch expected={} actual={}", hex::encode(expected), hex::encode(actual)),
        }
    }
}
impl std::error::Error for PersistenceError {}
impl From<io::Error> for PersistenceError { fn from(value: io::Error) -> Self { Self::Io(value) } }
impl From<bincode::Error> for PersistenceError { fn from(value: bincode::Error) -> Self { Self::Decode(value) } }
