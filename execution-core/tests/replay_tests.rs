use execution_core::{ExecutionNode, ExecutionPlan, VmInput};
use std::collections::BTreeMap;

fn sample_input() -> VmInput {
    VmInput {
        state: BTreeMap::from([("counter".to_string(), "7".to_string())]),
        plan: ExecutionPlan {
            nodes: vec![ExecutionNode {
                id: "n1".to_string(),
                action: "noop".to_string(),
                payload: serde_json::json!({}),
                deps: vec![],
            }],
        },
    }
}

#[test]
fn test_repeated_execution_replay() {
    let baseline = execution_core::execute::execute_vm(sample_input());
    for _ in 0..1000 {
        let out = execution_core::execute::execute_vm(sample_input());
        assert_eq!(out.receipt.receipt_hash, baseline.receipt.receipt_hash);
        assert_eq!(out.receipt.execution_root, baseline.receipt.execution_root);
        assert_eq!(out.receipt.new_state_root, baseline.receipt.new_state_root);
    }
}

#[test]
fn test_cross_verifier_replay() {
    let out = execution_core::execute::execute_vm(sample_input());
    let recomputed = execution_core::hashing::compute_receipt_hash(&out.receipt);
    assert_eq!(recomputed, out.receipt.receipt_hash);
}

#[test]
fn test_replay_after_restart() {
    let first = execution_core::execute::execute_vm(sample_input());
    let second = execution_core::execute::execute_vm(sample_input());
    assert_eq!(first.receipt.receipt_hash, second.receipt.receipt_hash);
}
