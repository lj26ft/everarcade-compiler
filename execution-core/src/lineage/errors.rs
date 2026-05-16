use std::{fmt, io};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineageMismatch {
    pub field: &'static str,
    pub index: usize,
    pub expected: String,
    pub actual: String,
}

#[derive(Debug)]
pub enum LineageError {
    Io(io::Error),
    Encode(bincode::Error),
    Decode(bincode::Error),
    Validation(LineageMismatch),
}

impl fmt::Display for LineageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::Encode(e) => write!(f, "{e}"),
            Self::Decode(e) => write!(f, "{e}"),
            Self::Validation(m) => write!(
                f,
                "field={} index={} expected={} actual={}",
                m.field, m.index, m.expected, m.actual
            ),
        }
    }
}

impl std::error::Error for LineageError {}

impl From<io::Error> for LineageError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
