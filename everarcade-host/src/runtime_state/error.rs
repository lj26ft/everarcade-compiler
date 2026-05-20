use std::{fmt, io};

#[derive(Debug)]
pub enum RuntimeStateError {
    Io(io::Error),
    Serde(serde_json::Error),
    Invalid(String),
}

impl fmt::Display for RuntimeStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::Serde(e) => write!(f, "serialization error: {e}"),
            Self::Invalid(e) => write!(f, "invalid runtime state: {e}"),
        }
    }
}

impl From<io::Error> for RuntimeStateError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<serde_json::Error> for RuntimeStateError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
