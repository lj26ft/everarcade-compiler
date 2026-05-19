use super::checkpoint::FinalizedCheckpoint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettlementAnchor {
    pub checkpoint_hash: String,
    pub xrpl_tx_hash: String,
    pub confirmed: bool,
}

pub fn create_settlement_anchor(
    checkpoint: &FinalizedCheckpoint,
    xrpl_tx_hash: String,
) -> SettlementAnchor {
    SettlementAnchor {
        checkpoint_hash: format!(
            "{:?}:{:?}:{}:{}",
            checkpoint.checkpoint_root,
            checkpoint.execution_id,
            checkpoint.finalized_tick,
            checkpoint.acknowledged_observers.len()
        ),
        xrpl_tx_hash,
        confirmed: true,
    }
}
