use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum NetworkError {
    Io(std::io::Error),
    Framing(String),
    Protocol(String),
}

impl Display for NetworkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "io error: {err}"),
            Self::Framing(msg) => write!(f, "framing error: {msg}"),
            Self::Protocol(msg) => write!(f, "protocol error: {msg}"),
        }
    }
}

impl From<std::io::Error> for NetworkError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
