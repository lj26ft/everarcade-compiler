use std::collections::BTreeMap;

use execution_core::{
    hash_runtime::state_hash::state_root, replay::replay_from_genesis, replay::DivergenceReason,
    ExecutionReceipt, StateChange, ABI_VERSION,
};

#[test]
fn tampered_next_root_detected() {
    let genesis = BTreeMap::new();
    let prior = state_root(&genesis);
    let receipt = ExecutionReceipt {
        protocol_epoch: 1,
        abi_version: ABI_VERSION.into(),
        contract_hash: "c".into(),
        input_hash: "i".into(),
        previous_state_root: prior,
        new_state_root: "tampered".into(),
        execution_root: "e".into(),
        fuel_used: 0,
        memory_used: 0,
        node_hashes: BTreeMap::new(),
        state_changes: vec![StateChange {
            key: "k".into(),
            before: "".into(),
            after: "v".into(),
        }],
        output_hash: "o".into(),
        receipt_hash: "r1".into(),
        snapshot_hash: "s".into(),
        previous_snapshot_hash: None,
    };
    let out = replay_from_genesis(genesis, &[receipt]);
    assert_eq!(
        out.divergence,
        Some((0, DivergenceReason::NextRootMismatch))
    );
}
