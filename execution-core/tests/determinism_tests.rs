use execution_core::{hashing, ExecutionNode, ExecutionPlan, VmInput};
use std::collections::BTreeMap;

fn sample_input() -> VmInput {
    VmInput {
        protocol_epoch_id: 1,
        state: BTreeMap::from([("counter".to_string(), "1".to_string())]),
        plan: ExecutionPlan {
            nodes: vec![ExecutionNode {
                id: "node-1".to_string(),
                action: "increment".to_string(),
                payload: serde_json::json!({"amount": 1}),
                deps: vec![],
            }],
        },
    }
}

#[test]
fn test_execution_root_stability() {
    let out1 = execution_core::execute::execute_vm(sample_input());
    let out2 = execution_core::execute::execute_vm(sample_input());
    assert_eq!(out1.receipt.execution_root, out2.receipt.execution_root);
}

#[test]
fn test_receipt_hash_stability() {
    let out1 = execution_core::execute::execute_vm(sample_input());
    let out2 = execution_core::execute::execute_vm(sample_input());
    assert_eq!(out1.receipt.receipt_hash, out2.receipt.receipt_hash);
}

#[test]
fn test_input_hash_stability() {
    let input = sample_input();
    let bytes = bincode::serialize(&input).unwrap();
    let a = hashing::hash_bytes(&bytes);
    let b = hashing::hash_bytes(&bytes);
    assert_eq!(a, b);
}
