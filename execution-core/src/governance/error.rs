#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum GovernanceError {
    #[error("proposal lineage divergence")]
    ProposalLineage,
    #[error("duplicate vote")]
    DuplicateVote,
    #[error("vote continuity divergence")]
    VoteContinuity,
    #[error("policy continuity divergence")]
    PolicyContinuity,
    #[error("authority continuity divergence")]
    AuthorityContinuity,
}
