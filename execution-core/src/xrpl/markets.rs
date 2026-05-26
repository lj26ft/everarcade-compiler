use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarketOffer {
    pub offer_id: String,
    pub seller: String,
    pub asset_id: String,
    pub amount_drops: u64,
}
