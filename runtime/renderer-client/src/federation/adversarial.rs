#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FederationAdversarialViolation {
    ReplayShardTampering,
    SynchronizationDivergence,
    StreamReplayInjection,
    ArchiveCorruption,
    ReplayChunkReordering,
    InvalidContinuityChains,
    ReplayCompressionCorruption,
    ReplayWindowForgery,
    AnchorMismatch,
    ObserverReplayDivergence,
}
