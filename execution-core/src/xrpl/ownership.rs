use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OwnershipManifest {
    pub manifest_id: String,
    pub asset_id: String,
    pub owner: String,
    pub continuity_counter: u64,
}
