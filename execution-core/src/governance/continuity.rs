use super::{
    AuthorityState, GovernanceCheckpoint, GovernancePolicy, GovernanceProposal, GovernanceVote,
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceContinuity {
    pub proposals: Vec<GovernanceProposal>,
    pub votes: Vec<GovernanceVote>,
    pub policies: Vec<GovernancePolicy>,
    pub authorities: Vec<AuthorityState>,
    pub checkpoints: Vec<GovernanceCheckpoint>,
}
pub fn sync_governance_continuity(
    local: &GovernanceContinuity,
    remote: &GovernanceContinuity,
) -> GovernanceContinuity {
    if local.checkpoints.len() >= remote.checkpoints.len() {
        local.clone()
    } else {
        remote.clone()
    }
}
pub fn verify_federated_governance(a: &GovernanceContinuity, b: &GovernanceContinuity) -> bool {
    a.checkpoints.last() == b.checkpoints.last()
}
