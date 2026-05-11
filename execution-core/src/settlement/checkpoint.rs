use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementCheckpoint {
    pub protocol_version: String,
    pub state_root: String,
    pub snapshot_hash: String,
    pub receipt_root: String,
    pub execution_root: String,
    pub verifier_consensus_hash: String,
    pub xrpl_tx_hash: Option<String>,
}
