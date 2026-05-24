use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryMutation {
    pub tick: u64,
    pub owner_id: String,
    pub asset_id: String,
    pub delta: i64,
    pub scenario: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetContinuityRecord {
    pub asset_id: String,
    pub current_owner: String,
    pub last_tick: u64,
}
