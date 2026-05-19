use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum AuthorityError {
    #[error("epoch rollback")]
    EpochRollback,
    #[error("non-monotonic epoch")]
    NonMonotonicEpoch,
    #[error("previous epoch hash mismatch")]
    PreviousEpochHashMismatch,
    #[error("handoff source authority mismatch")]
    HandoffSourceMismatch,
    #[error("handoff destination authority mismatch")]
    HandoffDestinationMismatch,
    #[error("handoff epoch mismatch")]
    HandoffEpochMismatch,
    #[error("checkpoint continuity mismatch")]
    CheckpointContinuityMismatch,
    #[error("lineage continuity mismatch")]
    LineageContinuityMismatch,
    #[error("self handoff rejected")]
    SelfHandoffRejected,
    #[error("unauthorized execution")]
    UnauthorizedExecution,
    #[error("missing authority chain")]
    MissingAuthorityChain,
    #[error("authority chain divergence")]
    AuthorityChainDivergence,
    #[error("handoff disabled by rotation policy")]
    HandoffDisabled,
    #[error("single authority execution policy violated")]
    MultiWriterExecutionRejected,
}
