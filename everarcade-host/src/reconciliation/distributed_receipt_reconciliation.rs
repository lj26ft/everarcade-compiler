use crate::distributed_receipts::receipt_store::DistributedExecutionReceipt;

pub fn reconcile_receipts(
    local: &[DistributedExecutionReceipt],
    remote: &[DistributedExecutionReceipt],
) -> Vec<DistributedExecutionReceipt> {
    if remote.len() > local.len() {
        remote.to_vec()
    } else {
        local.to_vec()
    }
}
