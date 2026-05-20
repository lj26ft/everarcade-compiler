use thiserror::Error;

#[derive(Debug, Error)]
pub enum FederationRuntimeError {
    #[error("invalid topology state")]
    InvalidTopologyState,
    #[error("continuity mismatch")]
    ContinuityMismatch,
    #[error("divergence detected: {0}")]
    DivergenceDetected(&'static str),
    #[error("canonical serialization error: {0}")]
    Serialization(String),
    #[error("continuity verification failed")]
    ContinuityVerificationFailed,
}
