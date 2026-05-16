use std::{fmt, io};

#[derive(Debug)]
pub enum StateError {
    DuplicateKey { key: Vec<u8> },
    BeforeMismatch { key: Vec<u8>, expected: Vec<u8>, actual: Vec<u8> },
    Codec(String),
    RootMismatch { expected: [u8; 32], actual: [u8; 32] },
    Io(io::Error),
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateKey { key } => write!(f, "field=state_diff key={} expected=unique actual=duplicate", hex::encode(key)),
            Self::BeforeMismatch { key, expected, actual } => write!(f, "field=state_before key={} expected={} actual={}", hex::encode(key), hex::encode(expected), hex::encode(actual)),
            Self::Codec(e) => write!(f, "{e}"),
            Self::RootMismatch { expected, actual } => write!(f, "field=state_root expected={} actual={}", hex::encode(expected), hex::encode(actual)),
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}
impl std::error::Error for StateError {}
impl From<bincode::Error> for StateError { fn from(value: bincode::Error) -> Self { Self::Codec(value.to_string()) } }
impl From<io::Error> for StateError { fn from(value: io::Error) -> Self { Self::Io(value) } }
