use serde::{Deserialize, Serialize};

use crate::federation::node::FederationNodeId;

use super::{
    errors::ConsensusError, policy::verify_consensus_policy, proposal::verify_consensus_proposal,
    quorum::verify_consensus_quorum, state::verify_consensus_state, ConsensusPolicy,
    ConsensusProposal, ConsensusQuorum, ConsensusRegistry, ConsensusState,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusVerificationReport {
    pub valid: bool,
    pub quorum_valid: bool,
}

pub fn verify_consensus(
    registry: &ConsensusRegistry,
    proposal: &ConsensusProposal,
    checkpoint_root: [u8; 32],
    quorum: &ConsensusQuorum,
    participating_nodes: &[FederationNodeId],
    policy: &ConsensusPolicy,
    state: &ConsensusState,
) -> Result<ConsensusVerificationReport, ConsensusError> {
    verify_consensus_proposal(proposal, checkpoint_root)?;
    verify_consensus_quorum(quorum, participating_nodes)?;
    verify_consensus_policy(policy, registry)?;
    verify_consensus_state(state, registry)?;
    Ok(ConsensusVerificationReport {
        valid: true,
        quorum_valid: true,
    })
}
