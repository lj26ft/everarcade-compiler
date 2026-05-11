use crate::{hashing, state_engine, ExecutionReceipt, VmInput, VmOutput, ABI_VERSION};
use std::collections::BTreeMap;

pub fn execute_vm(input: VmInput) -> VmOutput {
    let mut store = state_engine::store::StateStore::new(input.state.clone());
    let previous_state_root = state_engine::merkle::to_hex(&store.root());

    let node_hashes: BTreeMap<String, String> = input
        .plan
        .nodes
        .iter()
        .map(|n| (n.id.clone(), hashing::compute_node_hash(n)))
        .collect();
    let execution_root = hashing::compute_execution_root(&node_hashes);

    let (new_state_root, snapshot) = state_engine::apply::apply_state_changes(&mut store, &[], None);
    let state = store.into_state();

    let mut receipt = ExecutionReceipt {
        protocol_epoch: input.protocol_epoch_id,
        abi_version: ABI_VERSION.to_string(),
        contract_hash: String::new(),
        input_hash: hashing::hash_bytes(&bincode::serialize(&input).expect("input serialize failed")),
        previous_state_root,
        new_state_root,
        execution_root,
        fuel_used: 0,
        memory_used: 0,
        node_hashes,
        state_changes: vec![],
        output_hash: String::new(),
        receipt_hash: String::new(),
        snapshot_hash: snapshot.snapshot_hash.clone(),
        previous_snapshot_hash: snapshot.previous_snapshot_hash.clone(),
    };
    receipt.output_hash = hashing::hash_bytes(&bincode::serialize(&state).expect("state serialize failed"));
    receipt.receipt_hash = hashing::compute_receipt_hash(&receipt);
    VmOutput { updated_state: state, receipt }
}
