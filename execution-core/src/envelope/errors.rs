use thiserror::Error;
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum EnvelopeError {
    #[error("message id mismatch")]
    MessageIdMismatch,
    #[error("payload hash invalid")]
    PayloadHashInvalid,
    #[error("signature signer mismatch")]
    SignatureSignerMismatch,
    #[error("signature hash invalid")]
    SignatureHashInvalid,
    #[error("duplicate message")]
    DuplicateMessage,
    #[error("replay detected")]
    ReplayDetected,
    #[error("quarantine violation")]
    QuarantineViolation,
    #[error("policy violation")]
    PolicyViolation,
    #[error("registry continuity mismatch")]
    RegistryContinuityMismatch,
    #[error("state mismatch")]
    StateMismatch,
}
