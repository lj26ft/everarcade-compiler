use everarcade_host::distributed_receipts::{
    checkpoint_binding::ReceiptCheckpointBinding,
    receipt_checkpoint_validation::validate_checkpoint_binding,
};
#[test]
fn checkpoint_binding_mismatch_rejected() {
    let a = ReceiptCheckpointBinding {
        partition_root: [1; 32],
        replay_root: [2; 32],
        checkpoint_root: [3; 32],
    };
    let b = ReceiptCheckpointBinding {
        partition_root: [1; 32],
        replay_root: [2; 32],
        checkpoint_root: [4; 32],
    };
    assert!(!validate_checkpoint_binding(&a, &b));
}
