use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum LeaseError {
    #[error("invalid lease window: start > end")]
    InvalidWindow,
    #[error("non-monotonic lease window")]
    NonMonotonicWindow,
    #[error("overlapping lease windows")]
    OverlappingWindow,
    #[error("lease authority mismatch")]
    AuthorityMismatch,
    #[error("epoch continuity mismatch")]
    EpochContinuityMismatch,
    #[error("lease expired")]
    LeaseExpired,
    #[error("lease not yet active")]
    LeaseNotYetActive,
    #[error("lease renewal hash mismatch")]
    RenewalHashMismatch,
    #[error("lease continuity mismatch")]
    RenewalContinuityMismatch,
    #[error("lease policy violation")]
    PolicyViolation,
}
