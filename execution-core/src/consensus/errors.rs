use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ConsensusError {
    #[error("epoch rollback")]
    EpochRollback,
    #[error("non-monotonic epoch")]
    NonMonotonicEpoch,
    #[error("previous epoch hash mismatch")]
    PreviousEpochHashMismatch,
    #[error("proposal id mismatch")]
    ProposalIdMismatch,
    #[error("checkpoint continuity mismatch")]
    CheckpointContinuityMismatch,
    #[error("duplicate proposal")]
    DuplicateProposal,
    #[error("invalid quorum")]
    InvalidQuorum,
    #[error("policy violation")]
    PolicyViolation,
    #[error("registry continuity mismatch")]
    RegistryContinuityMismatch,
}
