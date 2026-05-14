use std::{fmt::Display, io};

#[derive(Debug)]
pub enum HostError {
    MissingPackage,
    InvalidPackage,
    InvalidReceipt,
    InvalidCheckpoint,
    InvalidStateFolder,
    IoError(io::Error),
    VerificationFailed(String),
    AnchorIntentMissing,
    InvalidArgs(String),
    Encode(bincode::Error),
    Decode(bincode::Error),
}

impl Display for HostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingPackage => write!(f, "missing package"),
            Self::InvalidPackage => write!(f, "invalid package"),
            Self::InvalidReceipt => write!(f, "invalid receipt"),
            Self::InvalidCheckpoint => write!(f, "invalid checkpoint"),
            Self::InvalidStateFolder => write!(f, "invalid state folder layout"),
            Self::IoError(e) => write!(f, "io error: {e}"),
            Self::VerificationFailed(e) => write!(f, "verification failed: {e}"),
            Self::AnchorIntentMissing => write!(f, "anchor intent missing"),
            Self::InvalidArgs(e) => write!(f, "invalid args: {e}"),
            Self::Encode(e) => write!(f, "encode error: {e}"),
            Self::Decode(e) => write!(f, "decode error: {e}"),
        }
    }
}

impl std::error::Error for HostError {}
impl From<io::Error> for HostError {
    fn from(v: io::Error) -> Self {
        Self::IoError(v)
    }
}
impl From<bincode::Error> for HostError {
    fn from(v: bincode::Error) -> Self {
        Self::Decode(v)
    }
}
