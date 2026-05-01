use crate::types::*;
use crate::hashing::*;
use bincode;

pub fn execute_vm(input: VmInput) -> VmOutput {
    let state_before = input.state.clone();

    let state_root_before =
        hash_state(&bincode::serialize(&state_before).unwrap());

    let mut state = state_before;
    let mut node_hashes = vec![];
    let mut changes = vec![];

    for node in input.plan.nodes.iter() {
        let before = state.clone();

        // deterministic contract dispatch
        match node.contract.as_str() {
            "set" => {
                state.insert("counter".into(), "5".into());
            }
            "increment" => {
                let v = state.get("counter")
                    .unwrap_or(&"0".to_string())
                    .parse::<i64>()
                    .unwrap_or(0) + 1;

                state.insert("counter".into(), v.to_string());
            }
            _ => {}
        }

        let after = state.clone();

        changes.push(StateChange {
            key: "counter".into(),
            before: before.get("counter").cloned().unwrap_or_default(),
            after: after.get("counter").cloned().unwrap_or_default(),
        });

        node_hashes.push(hash_execution(node.contract.as_bytes()));
    }

    let state_root_after =
        hash_state(&bincode::serialize(&state).unwrap());

    let execution_root = hash_execution(&node_hashes.join("|").as_bytes());

    let receipt = ExecutionReceipt {
        state_root_before,
        state_root_after,
        execution_root: execution_root.clone(),
        receipt_hash: hash_receipt(execution_root.as_bytes()),
        node_hashes,
        state_changes: changes,
    };

    VmOutput {
        updated_state: state,
        receipt,
    }
}
