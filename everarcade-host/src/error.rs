use std::{fmt::Display, io};

#[derive(Debug)]
pub enum HostError {
    Io(io::Error),
    Encode(bincode::Error),
    Decode(bincode::Error),
    InvalidReceipt,
    InvalidArgs(String),
}

impl Display for HostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::Encode(e) => write!(f, "encode error: {e}"),
            Self::Decode(e) => write!(f, "decode error: {e}"),
            Self::InvalidReceipt => write!(f, "invalid vm receipt"),
            Self::InvalidArgs(e) => write!(f, "invalid args: {e}"),
        }
    }
}

impl std::error::Error for HostError {}
impl From<io::Error> for HostError { fn from(v: io::Error) -> Self { Self::Io(v) } }
impl From<bincode::Error> for HostError { fn from(v: bincode::Error) -> Self { Self::Decode(v) } }
