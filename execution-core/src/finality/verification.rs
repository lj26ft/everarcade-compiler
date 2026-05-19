use serde::{Deserialize, Serialize};

use super::{
    ack::FinalityAcknowledgment,
    checkpoint::FinalizedCheckpoint,
    quorum::{verify_quorum, FinalityQuorum},
    registry::FinalityRegistry,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalityVerificationReport {
    pub valid: bool,
    pub rollback_detected: bool,
}

pub fn verify_finalization(
    checkpoint: &FinalizedCheckpoint,
    acknowledgements: &[FinalityAcknowledgment],
    quorum: &FinalityQuorum,
    registry: &FinalityRegistry,
) -> FinalityVerificationReport {
    let quorum_ok = verify_quorum(quorum, acknowledgements).is_ok();
    let rollback = checkpoint.finalized_tick < registry.latest_finalized_tick;
    FinalityVerificationReport {
        valid: quorum_ok && !rollback,
        rollback_detected: rollback,
    }
}
