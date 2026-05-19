use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{canonical::encoding::canonical_encode, operator::continuity::Hash256};

use super::{errors::ConsensusError, registry::ConsensusRegistry, ConsensusEpoch};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusState {
    pub current_epoch: ConsensusEpoch,
    pub registry_hash: Hash256,
}

pub fn hash_consensus_state(state: &ConsensusState) -> Hash256 {
    Sha256::digest(&canonical_encode(state).expect("consensus state encode")).into()
}

pub fn verify_consensus_state(
    state: &ConsensusState,
    registry: &ConsensusRegistry,
) -> Result<(), ConsensusError> {
    if state.current_epoch != registry.active_epoch {
        return Err(ConsensusError::RegistryContinuityMismatch);
    }
    if state.registry_hash != super::registry::hash_consensus_registry(registry) {
        return Err(ConsensusError::RegistryContinuityMismatch);
    }
    Ok(())
}
