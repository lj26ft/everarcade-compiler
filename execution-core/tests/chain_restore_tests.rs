use execution_core::{
    continuity::{restore_lineage_chain, ChainRestoreError, ChainRestoreInput},
    lineage::{ExecutionLineageChain, ExecutionLineageRecord},
    vm::{execute_vm_boundary, VmExecutionInput},
};

fn h(v: u8) -> [u8; 32] {
    [v; 32]
}

fn fixture_two_step() -> (tempfile::TempDir, ChainRestoreInput, ExecutionLineageChain) {
    let temp = tempfile::tempdir().unwrap();
    let package_path = temp.path().join("world.wasm");
    let checkpoint_path = temp.path().join("checkpoint_0.bin");
    let lineage_path = temp.path().join("lineage.bin");
    let receipt_1_path = temp.path().join("receipt_1.bin");
    let receipt_2_path = temp.path().join("receipt_2.bin");

    let package_bytes = vec![1u8; 64];
    std::fs::write(&package_path, &package_bytes).unwrap();
    let package_root = execution_core::persistence::package_store::package_root(&package_bytes);

    let mut checkpoint_state = execution_core::state::CanonicalState::default();
    checkpoint_state
        .entries
        .insert(b"__replay_root__".to_vec(), hex::encode(h(7)).into_bytes());
    let checkpoint_0 = execution_core::state::encode_checkpoint(&checkpoint_state).unwrap();
    std::fs::write(&checkpoint_path, &checkpoint_0).unwrap();
    let state0 = checkpoint_state.root();

    let i1 = VmExecutionInput {
        package_manifest_root: package_root,
        civilization_root: package_root,
        replay_root: state0,
        checkpoint_root: h(31),
        payload_root: h(31),
    };
    let (r1, _) = execute_vm_boundary(&i1);
    let mut state1 = checkpoint_state.clone();
    execution_core::state::apply_diff(&mut state1, &r1.state_diff).unwrap();
    let state1_root = state1.root();
    let i2 = VmExecutionInput {
        package_manifest_root: package_root,
        civilization_root: package_root,
        replay_root: state1_root,
        checkpoint_root: h(32),
        payload_root: h(32),
    };
    let (r2, _) = execute_vm_boundary(&i2);
    let mut state2 = state1.clone();
    execution_core::state::apply_diff(&mut state2, &r2.state_diff).unwrap();
    let state2_root = state2.root();
    std::fs::write(&receipt_1_path, bincode::serialize(&r1).unwrap()).unwrap();
    std::fs::write(&receipt_2_path, bincode::serialize(&r2).unwrap()).unwrap();

    let lineage = ExecutionLineageChain {
        world_id: h(99),
        package_root,
        records: vec![
            ExecutionLineageRecord {
                sequence: 1,
                previous_execution_id: None,
                execution_id: r1.execution_root,
                pre_state_root: state0,
                post_state_root: state1_root,
                receipt_hash: r1.receipt_id,
                package_root,
            },
            ExecutionLineageRecord {
                sequence: 2,
                previous_execution_id: Some(r1.execution_root),
                execution_id: r2.execution_root,
                pre_state_root: state1_root,
                post_state_root: state2_root,
                receipt_hash: r2.receipt_id,
                package_root,
            },
        ],
    };
    std::fs::write(&lineage_path, bincode::serialize(&lineage).unwrap()).unwrap();

    let input = ChainRestoreInput {
        package_path,
        checkpoint_path,
        lineage_path,
        receipt_paths: vec![receipt_1_path, receipt_2_path],
    };
    (temp, input, lineage)
}

#[test]
fn test_chain_restore_valid_two_step() {
    let (_t, input, lineage) = fixture_two_step();
    let report = restore_lineage_chain(input).unwrap();
    assert!(report.restore_ok);
    assert_eq!(report.final_state_root, lineage.records[1].post_state_root);
}

#[test]
fn test_chain_restore_receipt_count_mismatch_fails() {
    let (_t, mut input, _lineage) = fixture_two_step();
    input.receipt_paths.pop();
    let err = restore_lineage_chain(input).unwrap_err();
    assert!(matches!(err, ChainRestoreError::Validation(m) if m.field == "receipt_count"));
}

#[test]
fn test_chain_restore_checkpoint_root_mismatch_fails() {
    let (_t, input, _lineage) = fixture_two_step();
    std::fs::write(&input.checkpoint_path, [1u8; 8]).unwrap();
    let err = restore_lineage_chain(input).unwrap_err();
    assert!(matches!(err, ChainRestoreError::Validation(m) if m.field == "checkpoint_root"));
}

#[test]
fn test_chain_restore_lineage_sequence_mismatch_fails() {
    let (_t, input, mut lineage) = fixture_two_step();
    lineage.records[1].sequence = 9;
    std::fs::write(&input.lineage_path, bincode::serialize(&lineage).unwrap()).unwrap();
    let err = restore_lineage_chain(input).unwrap_err();
    assert!(matches!(err, ChainRestoreError::Lineage(_)));
}

#[test]
fn test_chain_restore_receipt_execution_id_mismatch_fails() {
    let (_t, input, mut lineage) = fixture_two_step();
    lineage.records[0].execution_id = h(55);
    std::fs::write(&input.lineage_path, bincode::serialize(&lineage).unwrap()).unwrap();
    let err = restore_lineage_chain(input).unwrap_err();
    assert!(matches!(err, ChainRestoreError::Validation(m) if m.field == "execution_id"));
}

#[test]
fn test_chain_restore_receipt_state_link_mismatch_fails() {
    let (_t, input, _lineage) = fixture_two_step();
    let mut receipt: execution_core::vm::VmExecutionReceipt =
        bincode::deserialize(&std::fs::read(&input.receipt_paths[1]).unwrap()).unwrap();
    receipt.prior_replay_root = h(88);
    std::fs::write(
        &input.receipt_paths[1],
        bincode::serialize(&receipt).unwrap(),
    )
    .unwrap();
    let err = restore_lineage_chain(input).unwrap_err();
    assert!(matches!(err, ChainRestoreError::Validation(m) if m.field == "pre_state_root"));
}

#[test]
fn test_chain_restore_package_root_mismatch_fails() {
    let (_t, input, mut lineage) = fixture_two_step();
    lineage.package_root = h(77);
    lineage.records[0].package_root = h(77);
    lineage.records[1].package_root = h(77);
    std::fs::write(&input.lineage_path, bincode::serialize(&lineage).unwrap()).unwrap();
    let err = restore_lineage_chain(input).unwrap_err();
    assert!(matches!(err, ChainRestoreError::Validation(m) if m.field == "package_root"));
}

#[test]
fn test_chain_restore_checkpoint_to_checkpoint_fixture_shape() {
    let (_t, input, lineage) = fixture_two_step();
    let report = restore_lineage_chain(input).unwrap();
    assert_eq!(
        report.expected_final_state_root,
        lineage.records[1].post_state_root
    );
}

#[test]
fn test_chain_restore_applies_diffs_to_reconstruct_final_root() {
    let (_t, input, lineage) = fixture_two_step();
    let report = restore_lineage_chain(input).unwrap();
    assert_eq!(report.final_state_root, lineage.records[1].post_state_root);
}

#[test]
fn test_chain_restore_state_diff_mismatch_fails() {
    let (_t, input, _lineage) = fixture_two_step();
    let mut receipt: execution_core::vm::VmExecutionReceipt =
        bincode::deserialize(&std::fs::read(&input.receipt_paths[0]).unwrap()).unwrap();
    receipt.state_diff[0].before = "bad".into();
    std::fs::write(
        &input.receipt_paths[0],
        bincode::serialize(&receipt).unwrap(),
    )
    .unwrap();
    let err = restore_lineage_chain(input).unwrap_err();
    assert!(matches!(err, ChainRestoreError::Validation(m) if m.field == "state_before"));
}

#[test]
fn test_fixture_chain_continuity() {
    let (_t, input, lineage) = fixture_two_step();
    let state0: execution_core::state::CanonicalState =
        execution_core::state::decode_checkpoint(&std::fs::read(&input.checkpoint_path).unwrap())
            .unwrap();
    let checkpoint_0_root = state0.root();
    assert_eq!(checkpoint_0_root, lineage.records[0].pre_state_root);

    let r1: execution_core::vm::VmExecutionReceipt =
        bincode::deserialize(&std::fs::read(&input.receipt_paths[0]).unwrap()).unwrap();
    let mut state1 = state0.clone();
    let checkpoint_1_root = execution_core::state::apply_diff(&mut state1, &r1.state_diff).unwrap();
    assert_eq!(checkpoint_1_root, lineage.records[0].post_state_root);
    assert_eq!(checkpoint_1_root, lineage.records[1].pre_state_root);

    let r2: execution_core::vm::VmExecutionReceipt =
        bincode::deserialize(&std::fs::read(&input.receipt_paths[1]).unwrap()).unwrap();
    let mut state2 = state1.clone();
    let checkpoint_2_root = execution_core::state::apply_diff(&mut state2, &r2.state_diff).unwrap();
    assert_eq!(checkpoint_2_root, lineage.records[1].post_state_root);
}
