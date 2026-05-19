use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{canonical::encoding::canonical_encode, operator::continuity::Hash256};

use super::errors::ConsensusError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusEpoch {
    pub epoch_number: u64,
    pub previous_epoch_hash: Hash256,
}

pub fn hash_consensus_epoch(epoch: &ConsensusEpoch) -> Hash256 {
    Sha256::digest(&canonical_encode(epoch).expect("consensus epoch encode")).into()
}

pub fn verify_consensus_epoch(
    previous: &ConsensusEpoch,
    next: &ConsensusEpoch,
) -> Result<(), ConsensusError> {
    if next.epoch_number <= previous.epoch_number {
        return Err(ConsensusError::EpochRollback);
    }
    if next.epoch_number != previous.epoch_number + 1 {
        return Err(ConsensusError::NonMonotonicEpoch);
    }
    if next.previous_epoch_hash != hash_consensus_epoch(previous) {
        return Err(ConsensusError::PreviousEpochHashMismatch);
    }
    Ok(())
}
