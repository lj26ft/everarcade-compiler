use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MigrationRecord {
    pub entity_id: String,
    pub source_region: String,
    pub target_region: String,
    pub ownership_epoch: u64,
    pub continuity_proof: String,
    pub sequence: u64,
}
