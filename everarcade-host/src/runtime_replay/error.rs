use std::{fmt, io};

#[derive(Debug)]
pub enum RuntimeReplayError {
    Io(io::Error),
    Serde(serde_json::Error),
    Invalid(String),
}

impl fmt::Display for RuntimeReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::Serde(e) => write!(f, "serialization error: {e}"),
            Self::Invalid(e) => write!(f, "runtime replay invalid: {e}"),
        }
    }
}
impl From<io::Error> for RuntimeReplayError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<serde_json::Error> for RuntimeReplayError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
impl From<crate::runtime_state::RuntimeStateError> for RuntimeReplayError {
    fn from(value: crate::runtime_state::RuntimeStateError) -> Self {
        Self::Invalid(value.to_string())
    }
}
