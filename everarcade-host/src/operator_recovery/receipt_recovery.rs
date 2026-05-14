use crate::distributed_receipts::receipt_store::DistributedExecutionReceipt;

pub fn recover_receipts(
    receipts: &[DistributedExecutionReceipt],
) -> Vec<DistributedExecutionReceipt> {
    receipts.to_vec()
}
