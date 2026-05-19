use serde::{Deserialize, Serialize};

use super::{errors::ConsensusError, registry::ConsensusRegistry};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusPolicy {
    pub proposals_required: bool,
}

pub fn verify_consensus_policy(
    policy: &ConsensusPolicy,
    registry: &ConsensusRegistry,
) -> Result<(), ConsensusError> {
    if policy.proposals_required && registry.active_proposals.is_empty() {
        return Err(ConsensusError::PolicyViolation);
    }
    Ok(())
}
