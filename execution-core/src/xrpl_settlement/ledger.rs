use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerCheckpoint {
    pub ledger_index: u64,
    pub settlement_epoch: u64,
    pub ledger_hash: [u8; 32],
}

pub fn verify_ledger_checkpoint(checkpoint: &LedgerCheckpoint) -> bool {
    checkpoint.ledger_index > 0 && checkpoint.ledger_hash != [0u8; 32]
}
