use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdkError {
    NonDeterministicMutation,
    ReplayMutationRejected,
    UnauthorizedAuthorityMutation,
    InvalidRuntimeConfiguration(String),
    DivergenceDetected { expected: String, actual: String },
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonDeterministicMutation => {
                write!(f, "non-deterministic runtime mutation rejected")
            }
            Self::ReplayMutationRejected => {
                write!(f, "replay mutation rejected: replay is reconstruction-only")
            }
            Self::UnauthorizedAuthorityMutation => {
                write!(f, "unauthorized authority mutation rejected")
            }
            Self::InvalidRuntimeConfiguration(msg) => {
                write!(f, "invalid runtime configuration: {msg}")
            }
            Self::DivergenceDetected { expected, actual } => write!(
                f,
                "deterministic divergence detected: expected {expected}, actual {actual}"
            ),
        }
    }
}

impl std::error::Error for SdkError {}
