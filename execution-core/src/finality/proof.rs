use serde::{Deserialize, Serialize};

use crate::operator::continuity::Hash256;

use super::{checkpoint::FinalizedCheckpoint, errors::FinalityError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalityProof {
    pub finalized_checkpoint_hash: Hash256,
    pub finalized_tick: u64,
}

pub fn verify_finality_proof(
    proof: &FinalityProof,
    checkpoint: &FinalizedCheckpoint,
) -> Result<(), FinalityError> {
    let expected_hash = super::checkpoint::hash_finalized_checkpoint(checkpoint);
    if proof.finalized_checkpoint_hash != expected_hash {
        return Err(FinalityError::CheckpointMismatch);
    }
    if proof.finalized_tick != checkpoint.finalized_tick {
        return Err(FinalityError::InvalidWindow);
    }
    Ok(())
}
