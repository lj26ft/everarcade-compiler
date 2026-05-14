use execution_core::{
    replay::{replay_from_genesis, DivergenceReason},
    ExecutionReceipt, ABI_VERSION,
};
use std::collections::BTreeMap;

#[test]
fn tampered_receipt_parent_detected() {
    let genesis = BTreeMap::new();
    let receipt = ExecutionReceipt {
        protocol_epoch: 1,
        abi_version: ABI_VERSION.into(),
        contract_hash: "c".into(),
        input_hash: "i".into(),
        previous_state_root: execution_core::hash_runtime::state_hash::state_root(&genesis),
        new_state_root: execution_core::hash_runtime::state_hash::state_root(&genesis),
        execution_root: "e".into(),
        fuel_used: 0,
        memory_used: 0,
        node_hashes: BTreeMap::new(),
        state_changes: vec![],
        output_hash: "o".into(),
        receipt_hash: "r1".into(),
        snapshot_hash: "s".into(),
        previous_snapshot_hash: Some("bad-parent".into()),
    };
    let out = replay_from_genesis(genesis, &[receipt]);
    assert_eq!(
        out.divergence,
        Some((0, DivergenceReason::ParentReceiptMismatch))
    );
}
