use crate::distributed_receipts::checkpoint_binding::ReceiptCheckpointBinding;

pub fn validate_checkpoint_binding(
    expected: &ReceiptCheckpointBinding,
    actual: &ReceiptCheckpointBinding,
) -> bool {
    expected == actual
}
