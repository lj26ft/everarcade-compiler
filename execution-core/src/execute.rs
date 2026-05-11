use crate::{hashing, ExecutionReceipt, VmInput, VmOutput, ABI_VERSION};
use std::collections::BTreeMap;

pub fn execute_vm(input: VmInput) -> VmOutput {
    let state = input.state.clone();
    let previous_state_root = hashing::compute_state_root(&state);
    let node_hashes: BTreeMap<String, String> = input
        .plan
        .nodes
        .iter()
        .map(|n| (n.id.clone(), hashing::compute_node_hash(n)))
        .collect();
    let execution_root = hashing::compute_execution_root(&node_hashes);

    let mut receipt = ExecutionReceipt {
        abi_version: ABI_VERSION.to_string(),
        contract_hash: String::new(),
        contract_hashes: BTreeMap::new(),
        input_hash: hashing::hash_bytes(&bincode::serialize(&input).expect("input serialize failed")),
        previous_state_root,
        new_state_root: hashing::compute_state_root(&state),
        execution_root,
        fuel_used: 0,
        memory_used: 0,
        node_hashes,
        state_changes: vec![],
        output_hash: String::new(),
        receipt_hash: String::new(),
    };
    receipt.output_hash = hashing::hash_bytes(&bincode::serialize(&state).expect("state serialize failed"));
    receipt.receipt_hash = hashing::compute_receipt_hash(&receipt);
    VmOutput { updated_state: state, receipt }
}
