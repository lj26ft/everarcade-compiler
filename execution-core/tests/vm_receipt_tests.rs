use execution_core::vm::vm_receipt::{
    compute_vm_receipt_root, validate_vm_receipt, VmExecutionReceipt,
};

#[test]
fn vm_receipt_validates() {
    let mut receipt = VmExecutionReceipt {
        receipt_id: [0; 32],
        package_root: [1; 32],
        prior_replay_root: [2; 32],
        next_replay_root: [3; 32],
        execution_root: [4; 32],
        checkpoint_root: [5; 32],
        anchor_root: [6; 32],
        state_diff: vec![],
    };
    receipt.receipt_id = compute_vm_receipt_root(&receipt);
    assert!(validate_vm_receipt(&receipt));
}
