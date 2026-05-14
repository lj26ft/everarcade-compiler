use super::{
    anchor_exchange::AnchorExchange, checkpoint_exchange::CheckpointExchange,
    receipt_exchange::ReceiptExchange,
};

pub fn imported_artifacts_consistent(
    receipts: &ReceiptExchange,
    checkpoints: &CheckpointExchange,
    anchors: &AnchorExchange,
) -> bool {
    !receipts.receipt_ids.is_empty()
        && !checkpoints.checkpoint_roots.is_empty()
        && !anchors.anchor_ids.is_empty()
}
