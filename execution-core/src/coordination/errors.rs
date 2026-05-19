use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum CoordinationError {
    #[error("session id mismatch")]
    SessionIdMismatch,
    #[error("empty participants")]
    EmptyParticipants,
    #[error("duplicate session")]
    DuplicateSession,
    #[error("proposal missing")]
    ProposalMissing,
    #[error("duplicate exchange")]
    DuplicateExchange,
    #[error("exchange actor not in session")]
    ExchangeActorNotInSession,
    #[error("policy violation")]
    PolicyViolation,
    #[error("quarantine violation")]
    QuarantineViolation,
    #[error("registry continuity mismatch")]
    RegistryContinuityMismatch,
}
