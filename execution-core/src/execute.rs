// FILE: execution-core/src/execute.rs
//
// MINIMAL DETERMINISTIC VM EXECUTOR
//
// PURPOSE:
// - restore compilation
// - establish stable VM pipeline
// - deterministic state execution
// - deterministic receipt generation
//
// SAFE RULES:
// - no filesystem
// - no clock
// - no randomness
// - no threading
//

use crate::{
    hashing,
    scheduler,
    ExecutionReceipt,
    ExecutionNode,
    State,
    StateChange,
    VmInput,
    VmOutput,
};

use std::collections::BTreeMap;

pub fn execute_vm(input: VmInput) -> VmOutput {
    //
    // INITIAL STATE
    //

    let mut state = input.state.clone();

    //
    // COMPUTE PREVIOUS ROOT
    //

    let previous_state_root =
        hashing::compute_state_root(&state);

    //
    // SORT DAG
    //

    let ordered_nodes =
        scheduler::topological_sort(&input.plan.nodes);

    //
    // EXECUTE
    //

    let mut node_hashes = BTreeMap::new();

    let mut state_changes = Vec::<StateChange>::new();

    for node in ordered_nodes {
        execute_node(
            &node,
            &mut state,
            &mut state_changes,
            &mut node_hashes,
        );
    }

    //
    // FINAL ROOTS
    //

    let new_state_root =
        hashing::compute_state_root(&state);

    let execution_root =
        hashing::compute_execution_root(&node_hashes);

    let receipt_hash =
        hashing::compute_receipt_hash(
            &previous_state_root,
            &new_state_root,
            &execution_root,
        );

    //
    // RECEIPT
    //

    let receipt = ExecutionReceipt {
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

//
// ============================================================
// NODE EXECUTION
// ============================================================
//

fn execute_node(
    node: &ExecutionNode,
    state: &mut State,
    state_changes: &mut Vec<StateChange>,
    node_hashes: &mut BTreeMap<String, String>,
) {
    match node.action.as_str() {
        //
        // ====================================================
        // SET CONTRACT
        // ====================================================
        //

        "set" => {
            let key = node.payload["key"]
                .as_str()
                .expect("missing key")
                .to_string();

            let value = node.payload["value"]
                .as_str()
                .expect("missing value")
                .to_string();

            let before =
                state.get(&key)
                    .cloned()
                    .unwrap_or_default();

            state.insert(key.clone(), value.clone());

            state_changes.push(StateChange {
                key: key.clone(),
                before,
                after: value,
            });
        }

        //
        // ====================================================
        // INCREMENT CONTRACT
        // ====================================================
        //

        "increment" => {
            let key = node.payload["key"]
                .as_str()
                .expect("missing key")
                .to_string();

            let amount = node.payload["amount"]
                .as_i64()
                .expect("missing amount");

            let current =
                state.get(&key)
                    .cloned()
                    .unwrap_or_else(|| "0".to_string());

            let current_num: i64 =
                current.parse()
                    .expect("invalid integer");

            let next =
                current_num + amount;

            let next_string =
                next.to_string();

            state.insert(
                key.clone(),
                next_string.clone(),
            );

            state_changes.push(StateChange {
                key: key.clone(),
                before: current,
                after: next_string,
            });
        }

        //
        // ====================================================
        // UNKNOWN ACTION
        // ====================================================
        //

        _ => {
            panic!("unknown action: {}", node.action);
        }
    }

    //
    // NODE HASH
    //

    let node_hash =
        hashing::compute_node_hash(node);

    node_hashes.insert(
        node.id.clone(),
        node_hash,
    );
}
