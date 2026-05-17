use std::fmt::{Display, Formatter};

use super::continuity::OperatorRecoveryMismatch;

#[derive(Debug)]
pub enum OperatorRecoveryError {
    Validation(OperatorRecoveryMismatch),
    Storage(String),
}

impl Display for OperatorRecoveryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(m) => write!(
                f,
                "validation failed: {} expected={} actual={}",
                m.field, m.expected, m.actual
            ),
            Self::Storage(m) => write!(f, "storage error: {m}"),
        }
    }
}

impl std::error::Error for OperatorRecoveryError {}
