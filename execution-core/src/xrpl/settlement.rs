use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SettlementReceipt {
    pub world_id: String,
    pub settlement_epoch: u64,
    pub amount_drops: u64,
    pub destination: String,
    pub witness_hash: String,
}
