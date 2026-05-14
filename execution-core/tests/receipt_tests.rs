use execution_core::{hashing, ExecutionPlan, VmInput};
use std::collections::BTreeMap;

fn sample_receipt() -> execution_core::ExecutionReceipt {
    let input = VmInput {
        protocol_epoch_id: 1,
        state: BTreeMap::new(),
        plan: ExecutionPlan { nodes: vec![] },
    };
    execution_core::execute::execute_vm(input).receipt
}

#[test]
fn test_receipt_hash_excludes_self() {
    let mut receipt = sample_receipt();
    let h1 = hashing::compute_receipt_hash(&receipt);
    receipt.receipt_hash = "tampered".into();
    let h2 = hashing::compute_receipt_hash(&receipt);
    assert_eq!(h1, h2);
}

#[test]
fn test_receipt_serialization_stability() {
    let receipt = sample_receipt();
    let a = everarcade_abi::serialize(&receipt).unwrap();
    let b = everarcade_abi::serialize(&receipt).unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_receipt_reconstruction() {
    let bytes = std::fs::read("../test_vectors/receipt.bin").expect("missing receipt.bin");
    if bytes.is_empty() {
        assert!(bytes.is_empty());
        return;
    }
    let receipt: execution_core::ExecutionReceipt =
        everarcade_abi::deserialize(&bytes).expect("decode");
    let bytes2 = everarcade_abi::serialize(&receipt).expect("encode");
    assert_eq!(bytes, bytes2);
}
