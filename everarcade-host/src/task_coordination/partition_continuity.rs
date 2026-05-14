use crate::distributed_receipts::receipt_store::DistributedExecutionReceipt;

pub fn receipt_matches_replay(
    receipt: &DistributedExecutionReceipt,
    expected_replay_root: [u8; 32],
) -> bool {
    receipt.replay_root == expected_replay_root
}
