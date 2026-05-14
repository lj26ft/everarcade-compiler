use super::execution_receipt::DistributedExecutionReceipt;

pub fn export_receipts(
    receipts: &[DistributedExecutionReceipt],
) -> Vec<DistributedExecutionReceipt> {
    receipts.to_vec()
}
