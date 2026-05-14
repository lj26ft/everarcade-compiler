use crate::distributed_receipts::receipt_store::DistributedExecutionReceipt;

pub fn validate_imported_receipt(
    receipt: &DistributedExecutionReceipt,
    replay_verified: bool,
) -> bool {
    replay_verified && receipt.receipt_root != [0u8; 32]
}
