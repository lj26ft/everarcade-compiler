use super::{
    anchor_exchange::AnchorExchange, artifact_validation::imported_artifacts_consistent,
    checkpoint_exchange::CheckpointExchange, receipt_exchange::ReceiptExchange,
};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConvergenceResult {
    pub converged: bool,
}

pub fn evaluate(local_root: Hash, remote_root: Hash) -> ConvergenceResult {
    ConvergenceResult {
        converged: local_root == remote_root,
    }
}

pub fn converge_roots(
    receipts: &ReceiptExchange,
    checkpoints: &CheckpointExchange,
    anchors: &AnchorExchange,
) -> Result<String, String> {
    if !imported_artifacts_consistent(receipts, checkpoints, anchors) {
        return Err("incomplete artifact set".to_string());
    }
    Ok(format!(
        "{}:{}:{}",
        receipts.receipt_ids.len(),
        checkpoints.checkpoint_roots.len(),
        anchors.anchor_ids.len()
    ))
}
