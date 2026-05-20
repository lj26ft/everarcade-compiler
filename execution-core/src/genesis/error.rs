use std::fmt;

#[derive(Debug)]
pub enum GenesisError {
    Invalid(String),
    Serialization(String),
}

impl fmt::Display for GenesisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(v) => write!(f, "invalid genesis: {v}"),
            Self::Serialization(v) => write!(f, "genesis serialization failed: {v}"),
        }
    }
}

impl std::error::Error for GenesisError {}
