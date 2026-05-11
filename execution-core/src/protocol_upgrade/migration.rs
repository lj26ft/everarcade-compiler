use crate::state_engine::snapshot::StateSnapshot;

use super::{compatibility_matrix, epoch::ProtocolEpoch, transition::UpgradeTransition};

#[derive(Debug, Clone)]
pub struct MigrationResult {
    pub upgraded_snapshot: StateSnapshot,
    pub transformation_proof: String,
}

pub fn apply_migration(
    snapshot_before: StateSnapshot,
    from_epoch: &ProtocolEpoch,
    to_epoch: &ProtocolEpoch,
    transition: &UpgradeTransition,
) -> Option<MigrationResult> {
    if !compatibility_matrix::is_allowed_transition(from_epoch, to_epoch)
        || transition.from_epoch_id != from_epoch.epoch_id
        || transition.to_epoch_id != to_epoch.epoch_id
        || !transition.snapshot_compatible
        || !transition.receipt_compatible
        || !transition.deterministic_migration
    {
        return None;
    }

    Some(MigrationResult {
        upgraded_snapshot: snapshot_before,
        transformation_proof: transition.transition_hash.clone(),
    })
}
