use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EconomyMutation {
    pub tick: u64,
    pub asset_id: String,
    pub from_owner: String,
    pub to_owner: String,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EconomicLedgerCheckpoint {
    pub tick: u64,
    pub mutation_count: usize,
    pub ledger_root: String,
}
