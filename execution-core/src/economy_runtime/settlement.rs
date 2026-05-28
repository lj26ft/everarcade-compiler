use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EconomicSettlement {
    pub settlement_id: String,
    pub balance_root: String,
}
