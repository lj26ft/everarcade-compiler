use std::collections::BTreeMap;

use execution_core::{
    hash_runtime::state_hash::state_root, replay::replay_from_genesis, ExecutionReceipt,
    StateChange, ABI_VERSION,
};

fn mk_receipt(
    prev_hash: Option<String>,
    prev_root: String,
    new_root: String,
    changes: Vec<StateChange>,
) -> ExecutionReceipt {
    ExecutionReceipt {
        protocol_epoch: 1,
        abi_version: ABI_VERSION.into(),
        contract_hash: "c".into(),
        input_hash: "i".into(),
        previous_state_root: prev_root,
        new_state_root: new_root,
        execution_root: "e".into(),
        fuel_used: 1,
        memory_used: 1,
        node_hashes: BTreeMap::new(),
        state_changes: changes,
        output_hash: "o".into(),
        receipt_hash: "r".into(),
        snapshot_hash: "s".into(),
        previous_snapshot_hash: prev_hash,
    }
}

#[test]
fn same_genesis_and_receipts_same_final_root() {
    let mut genesis = BTreeMap::new();
    genesis.insert("x".to_string(), "1".to_string());
    let prior = state_root(&genesis);
    let mut changed = genesis.clone();
    changed.insert("x".to_string(), "2".to_string());
    let next = state_root(&changed);
    let receipts = vec![mk_receipt(
        None,
        prior,
        next,
        vec![StateChange {
            key: "x".into(),
            before: "1".into(),
            after: "2".into(),
        }],
    )];
    let a = replay_from_genesis(genesis.clone(), &receipts);
    let b = replay_from_genesis(genesis, &receipts);
    assert_eq!(a.final_state_root, b.final_state_root);
    assert!(a.divergence.is_none());
}
