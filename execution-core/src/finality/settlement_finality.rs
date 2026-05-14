use super::checkpoint::FinalityCheckpoint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettlementAnchor {
    pub checkpoint_hash: String,
    pub xrpl_tx_hash: String,
    pub confirmed: bool,
}

pub fn create_settlement_anchor(
    checkpoint: &FinalityCheckpoint,
    xrpl_tx_hash: String,
) -> SettlementAnchor {
    SettlementAnchor {
        checkpoint_hash: format!(
            "{}:{}:{}:{}",
            checkpoint.execution_root,
            checkpoint.receipt_root,
            checkpoint.snapshot_root,
            checkpoint.epoch_id
        ),
        xrpl_tx_hash,
        confirmed: true,
    }
}
