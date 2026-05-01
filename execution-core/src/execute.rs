use crate::{
    hashing,
    scheduler,
    ExecutionNode,
    State,
    StateChange,
    VmInput,
    VmOutput,
};

use std::collections::BTreeMap;

pub fn execute_vm(input: VmInput) -> VmOutput {
    let mut state = input.state.clone();

    let previous_state_root = hashing::compute_state_root(&state);

    let ordered_nodes =
        scheduler::topological_sort(&input.plan.nodes);

    let mut node_hashes = BTreeMap::new();
    let mut state_changes = Vec::new();

    for node in ordered_nodes {
        execute_node(
            &node,
            &mut state,
            &mut state_changes,
            &mut node_hashes,
        );
    }

    let new_state_root = hashing::compute_state_root(&state);
    let execution_root = hashing::compute_execution_root(&node_hashes);

    let receipt_hash =
        hashing::compute_receipt_hash(
            &previous_state_root,
            &new_state_root,
            &execution_root,
        );

    let receipt = crate::ExecutionReceipt {
        abi_version: crate::abi::ABI_VERSION.to_string(),
        previous_state_root,
        new_state_root,
        execution_root,
        receipt_hash,
        node_hashes,
        state_changes,
    };

    VmOutput {
        updated_state: state,
        receipt,
    }
}

fn execute_node(
    node: &ExecutionNode,
    state: &mut State,
    state_changes: &mut Vec<StateChange>,
    node_hashes: &mut BTreeMap<String, String>,
) {
    match node.action.as_str() {

        "set" => {
            let key = node
                .payload
                .get("key")
                .unwrap()
                .clone();

            let value = node
                .payload
                .get("value")
                .unwrap()
                .clone();

            let before = state.get(&key).cloned().unwrap_or_default();

            state.insert(key.clone(), value.clone());

            state_changes.push(StateChange {
                key,
                before,
                after: value,
            });
        }

        "increment" => {
            let key = node
                .payload
                .get("key")
                .unwrap()
                .clone();

            let amount_str = node
                .payload
                .get("amount")
                .unwrap();

            let amount: i64 =
                amount_str.parse().unwrap();

            let current =
                state.get(&key).cloned().unwrap_or("0".to_string());

            let current_num: i64 =
                current.parse().unwrap();

            let next = (current_num + amount).to_string();

            state.insert(key.clone(), next.clone());

            state_changes.push(StateChange {
                key,
                before: current,
                after: next,
            });
        }

        _ => panic!("unknown action"),
    }

    let hash = hashing::compute_node_hash(node);
    node_hashes.insert(node.id.clone(), hash);
}
