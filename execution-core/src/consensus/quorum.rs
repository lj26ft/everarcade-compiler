use crate::federation::node::FederationNodeId;

use super::errors::ConsensusError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsensusQuorum {
    pub required_nodes: usize,
}

pub fn verify_consensus_quorum(
    quorum: &ConsensusQuorum,
    participating_nodes: &[FederationNodeId],
) -> Result<(), ConsensusError> {
    let mut nodes = participating_nodes.to_vec();
    nodes.sort();
    nodes.dedup();
    if nodes.len() < quorum.required_nodes {
        return Err(ConsensusError::InvalidQuorum);
    }
    if nodes.len() != participating_nodes.len() {
        return Err(ConsensusError::DuplicateProposal);
    }
    Ok(())
}
