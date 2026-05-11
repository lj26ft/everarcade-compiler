use execution_core::{hashing, ExecutionNode, ExecutionPlan, VmInput};
use std::collections::BTreeMap;

fn sample_input() -> VmInput {
    VmInput {
        state: BTreeMap::from([("score".to_string(), "1".to_string())]),
        plan: ExecutionPlan {
            nodes: vec![ExecutionNode {
                id: "n1".to_string(),
                action: "increment".to_string(),
                payload: serde_json::json!({"key": "score"}),
                deps: vec![],
            }],
        },
    }
}

#[test]
fn test_state_root_determinism() {
    let state = sample_input().state;
    assert_eq!(hashing::compute_state_root(&state), hashing::compute_state_root(&state));
}

#[test]
fn test_execution_root_determinism() {
    let input = sample_input();
    let node_hashes: BTreeMap<String, String> = input
        .plan
        .nodes
        .iter()
        .map(|n| (n.id.clone(), hashing::compute_node_hash(n)))
        .collect();
    assert_eq!(
        hashing::compute_execution_root(&node_hashes),
        hashing::compute_execution_root(&node_hashes)
    );
}

#[test]
fn test_node_hash_determinism() {
    let node = &sample_input().plan.nodes[0];
    assert_eq!(hashing::compute_node_hash(node), hashing::compute_node_hash(node));
}

#[test]
fn test_receipt_hash_determinism() {
    let out = execution_core::execute::execute_vm(sample_input());
    assert_eq!(
        hashing::compute_receipt_hash(&out.receipt),
        hashing::compute_receipt_hash(&out.receipt)
    );
}
