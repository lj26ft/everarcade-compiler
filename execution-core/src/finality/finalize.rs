use serde::{Deserialize, Serialize};

use super::{
    ack::FinalityAcknowledgment,
    checkpoint::FinalizedCheckpoint,
    errors::FinalityError,
    quorum::{verify_quorum, FinalityQuorum},
    window::{verify_finalization_window, FinalizationWindow},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizationReport {
    pub finalized: bool,
    pub quorum_reached: bool,
}

pub fn finalize_checkpoint(
    checkpoint: &FinalizedCheckpoint,
    acknowledgements: &[FinalityAcknowledgment],
    quorum: &FinalityQuorum,
    window: &FinalizationWindow,
    previous_window: Option<&FinalizationWindow>,
) -> Result<FinalizationReport, FinalityError> {
    if checkpoint.checkpoint_root == [0u8; 32] {
        return Err(FinalityError::CheckpointMismatch);
    }
    if checkpoint.execution_id == [0u8; 32] {
        return Err(FinalityError::ExecutionMismatch);
    }
    verify_finalization_window(window, previous_window)?;
    verify_quorum(quorum, acknowledgements)?;
    Ok(FinalizationReport {
        finalized: true,
        quorum_reached: true,
    })
}
