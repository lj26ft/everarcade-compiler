use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegionOwnership {
    pub region_id: String,
    pub owner_node: String,
    pub epoch: u64,
    pub continuity_root: String,
}
