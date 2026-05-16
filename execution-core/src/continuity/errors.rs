use std::{fmt, io};

use crate::{lineage::LineageError, persistence::PersistenceError};

use super::chain_restore::ChainRestoreMismatch;

#[derive(Debug)]
pub enum ChainRestoreError {
    Io(io::Error),
    Persistence(PersistenceError),
    Lineage(LineageError),
    Validation(ChainRestoreMismatch),
}

impl fmt::Display for ChainRestoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::Persistence(e) => write!(f, "{e}"),
            Self::Lineage(e) => write!(f, "{e}"),
            Self::Validation(m) => write!(
                f,
                "field={} index={} expected={} actual={}",
                m.field,
                m.index
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "none".into()),
                m.expected,
                m.actual
            ),
        }
    }
}
impl std::error::Error for ChainRestoreError {}
impl From<io::Error> for ChainRestoreError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<PersistenceError> for ChainRestoreError {
    fn from(value: PersistenceError) -> Self {
        Self::Persistence(value)
    }
}
impl From<LineageError> for ChainRestoreError {
    fn from(value: LineageError) -> Self {
        Self::Lineage(value)
    }
}
