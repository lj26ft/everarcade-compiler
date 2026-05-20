use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegionState {
    pub region_id: String,
    pub continuity_root: String,
    pub epoch: u64,
}
