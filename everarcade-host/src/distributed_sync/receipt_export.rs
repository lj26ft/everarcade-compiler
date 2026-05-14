use crate::distributed_receipts::receipt_store::DistributedExecutionReceipt;

pub fn serialize_receipts(
    receipts: &[DistributedExecutionReceipt],
) -> Vec<DistributedExecutionReceipt> {
    receipts.to_vec()
}
