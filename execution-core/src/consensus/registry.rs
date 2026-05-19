use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{canonical::encoding::canonical_encode, operator::continuity::Hash256};

use super::{epoch::ConsensusEpoch, errors::ConsensusError, proposal::ConsensusProposal};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusRegistry {
    pub active_epoch: ConsensusEpoch,
    pub active_proposals: BTreeMap<Hash256, ConsensusProposal>,
}

pub fn hash_consensus_registry(registry: &ConsensusRegistry) -> Hash256 {
    Sha256::digest(&canonical_encode(registry).expect("consensus registry encode")).into()
}

pub fn update_consensus_registry(
    registry: &ConsensusRegistry,
    next_epoch: ConsensusEpoch,
    proposal: ConsensusProposal,
) -> Result<ConsensusRegistry, ConsensusError> {
    super::epoch::verify_consensus_epoch(&registry.active_epoch, &next_epoch)?;
    if registry
        .active_proposals
        .contains_key(&proposal.proposal_id)
    {
        return Err(ConsensusError::DuplicateProposal);
    }
    let mut next = registry.clone();
    next.active_epoch = next_epoch;
    next.active_proposals.insert(proposal.proposal_id, proposal);
    Ok(next)
}
