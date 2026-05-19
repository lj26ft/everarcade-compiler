use serde::{Deserialize, Serialize};

use crate::operator::continuity::Hash256;

use super::{checkpoint::FinalizedCheckpoint, errors::FinalityError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalityRegistry {
    pub latest_finalized_checkpoint: Hash256,
    pub latest_finalized_tick: u64,
}

pub fn update_finality_registry(
    registry: &FinalityRegistry,
    checkpoint: &FinalizedCheckpoint,
) -> Result<FinalityRegistry, FinalityError> {
    if checkpoint.finalized_tick < registry.latest_finalized_tick {
        return Err(FinalityError::FinalizedRollback);
    }
    Ok(FinalityRegistry {
        latest_finalized_checkpoint: checkpoint.checkpoint_root,
        latest_finalized_tick: checkpoint.finalized_tick,
    })
}
