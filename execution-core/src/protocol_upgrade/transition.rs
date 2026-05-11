use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeTransition {
    pub from_epoch_id: u64,
    pub to_epoch_id: u64,
    pub snapshot_compatible: bool,
    pub receipt_compatible: bool,
    pub deterministic_migration: bool,
    pub transition_hash: String,
}
