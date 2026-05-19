use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    canonical::encoding::canonical_encode, federation::node::FederationNodeId,
    operator::continuity::Hash256,
};

use super::errors::ConsensusError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusProposal {
    pub proposal_id: Hash256,
    pub checkpoint_root: Hash256,
    pub proposed_by: FederationNodeId,
}

pub fn hash_consensus_proposal(proposal: &ConsensusProposal) -> Hash256 {
    let payload = (proposal.checkpoint_root, proposal.proposed_by);
    Sha256::digest(&canonical_encode(&payload).expect("consensus proposal encode")).into()
}

pub fn verify_consensus_proposal(
    proposal: &ConsensusProposal,
    expected_checkpoint_root: Hash256,
) -> Result<(), ConsensusError> {
    if proposal.checkpoint_root != expected_checkpoint_root {
        return Err(ConsensusError::CheckpointContinuityMismatch);
    }
    if proposal.proposal_id != hash_consensus_proposal(proposal) {
        return Err(ConsensusError::ProposalIdMismatch);
    }
    Ok(())
}
