use super::receipt_store::{DistributedExecutionReceipt, Hash};

pub fn bind_receipt_to_checkpoint(
    receipt: &DistributedExecutionReceipt,
    checkpoint_root: Hash,
) -> (Hash, Hash) {
    (receipt.receipt_root, checkpoint_root)
}
