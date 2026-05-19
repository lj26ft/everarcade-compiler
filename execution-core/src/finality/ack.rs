use serde::{Deserialize, Serialize};

use crate::{federation::node::FederationNodeId, operator::continuity::Hash256};

use super::errors::FinalityError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalityAcknowledgment {
    pub observer: FederationNodeId,
    pub checkpoint_root: Hash256,
    pub execution_id: Hash256,
}

pub fn verify_ack(
    ack: &FinalityAcknowledgment,
    valid_observers: &[FederationNodeId],
    expected_checkpoint_root: Hash256,
    expected_execution_id: Hash256,
) -> Result<(), FinalityError> {
    if !valid_observers.contains(&ack.observer) {
        return Err(FinalityError::InvalidObserver);
    }
    if ack.checkpoint_root != expected_checkpoint_root {
        return Err(FinalityError::CheckpointMismatch);
    }
    if ack.execution_id != expected_execution_id {
        return Err(FinalityError::ExecutionMismatch);
    }
    Ok(())
}
