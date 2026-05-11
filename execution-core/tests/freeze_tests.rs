use execution_core::{
    freeze::{compatibility, governance, versions},
    hashing,
    ExecutionPlan, VmInput,
};

#[test]
fn test_abi_frozen() {
    assert_eq!(versions::ABI_VERSION, everarcade_abi::ABI_VERSION);
    assert!(compatibility::is_abi_compatible(everarcade_abi::ABI_VERSION));
}

#[test]
fn test_receipt_hash_frozen() {
    let input = VmInput { protocol_epoch_id: 1, state: Default::default(), plan: ExecutionPlan { nodes: vec![] } };
    let first = execution_core::execute::execute_vm(input.clone());
    let second = execution_core::execute::execute_vm(input);

    assert_eq!(hashing::compute_receipt_hash(&first.receipt), hashing::compute_receipt_hash(&second.receipt));
}

#[test]
fn test_snapshot_hash_frozen() {
    let input = VmInput { protocol_epoch_id: 1, state: Default::default(), plan: ExecutionPlan { nodes: vec![] } };
    let first = execution_core::execute::execute_vm(input.clone());
    let second = execution_core::execute::execute_vm(input);
    assert_eq!(first.receipt.snapshot_hash, second.receipt.snapshot_hash);
}

#[test]
fn test_execution_replay_frozen() {
    let input = VmInput { protocol_epoch_id: 1, state: Default::default(), plan: ExecutionPlan { nodes: vec![] } };
    let first = execution_core::execute::execute_vm(input.clone());
    let second = execution_core::execute::execute_vm(input);
    assert_eq!(first.receipt.execution_root, second.receipt.execution_root);
}

#[test]
fn test_protocol_upgrade_rejection() {
    let err = governance::validate_protocol_upgrade("everarcade-protocol-v2").unwrap_err();
    assert!(err.contains("incompatible protocol version"));
}
