use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetOwnership {
    pub asset_id: String,
    pub owner_id: String,
    pub settlement_epoch: u64,
}
