use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PartitionCheckpoint {
    pub partition_id: String,
    pub tick: u64,
    pub continuity_root: String,
}
