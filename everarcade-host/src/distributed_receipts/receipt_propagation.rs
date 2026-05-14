use super::receipt_store::DistributedExecutionReceipt;

pub fn export_receipts(
    receipts: &[DistributedExecutionReceipt],
) -> Vec<DistributedExecutionReceipt> {
    receipts.to_vec()
}
