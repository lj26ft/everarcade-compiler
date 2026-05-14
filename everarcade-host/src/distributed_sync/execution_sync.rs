use crate::distributed_receipts::receipt_store::DistributedExecutionReceipt;

pub fn sync_execution(
    receipts: &[DistributedExecutionReceipt],
) -> Vec<DistributedExecutionReceipt> {
    receipts.to_vec()
}
