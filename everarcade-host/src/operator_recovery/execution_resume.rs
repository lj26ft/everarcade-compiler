use crate::distributed_receipts::receipt_store::DistributedExecutionReceipt;

pub fn resume_execution(receipt: &DistributedExecutionReceipt, replay_root: [u8; 32]) -> bool {
    receipt.replay_root == replay_root
}
