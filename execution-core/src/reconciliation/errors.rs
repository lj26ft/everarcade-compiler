use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReconciliationError {
    DuplicateFork,
}

impl Display for ReconciliationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateFork => write!(f, "duplicate quarantined fork"),
        }
    }
}

impl std::error::Error for ReconciliationError {}
