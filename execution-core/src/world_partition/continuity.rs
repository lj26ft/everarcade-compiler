use serde::{Deserialize, Serialize};

use super::{MigrationRecord, RegionOwnership};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PartitionContinuity {
    pub lineage: Vec<String>,
    pub ownership_history: Vec<RegionOwnership>,
    pub migrations: Vec<MigrationRecord>,
}
