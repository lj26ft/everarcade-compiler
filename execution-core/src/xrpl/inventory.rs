use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventoryAnchor {
    pub player_id: String,
    pub inventory_root: String,
    pub tick: u64,
}
