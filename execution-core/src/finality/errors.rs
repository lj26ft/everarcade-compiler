use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum FinalityError {
    #[error("invalid observer")]
    InvalidObserver,
    #[error("checkpoint mismatch")]
    CheckpointMismatch,
    #[error("execution mismatch")]
    ExecutionMismatch,
    #[error("invalid finalization window")]
    InvalidWindow,
    #[error("overlapping finalization windows")]
    OverlappingWindow,
    #[error("duplicate acknowledgment")]
    DuplicateAcknowledgment,
    #[error("quorum not reached")]
    QuorumNotReached,
    #[error("finalized rollback detected")]
    FinalizedRollback,
}
