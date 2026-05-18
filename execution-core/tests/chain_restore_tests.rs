use execution_core::{
    continuity::{restore_lineage_chain, ChainRestoreError, ChainRestoreInput},
    lineage::{ExecutionLineageChain, ExecutionLineageRecord},
    vm::{execute_vm_boundary, genesis_replay_root_value, VmExecutionInput, REPLAY_ROOT_STATE_KEY},
};

fn h(v: u8) -> [u8; 32] {
    [v; 32]
}

fn decode_state_replay_root_bytes(raw: &[u8]) -> [u8; 32] {
    let hex_text = std::str::from_utf8(raw).expect("replay root state must be utf8 hex");
    let decoded = hex::decode(hex_text).expect("replay root state must be valid hex");
    decoded
        .as_slice()
        .try_into()
        .expect("replay root state must decode to 32 bytes")
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
    checkpoint_state.entries.insert(
        REPLAY_ROOT_STATE_KEY.as_bytes().to_vec(),
        genesis_replay_root_value(),
    );
    let checkpoint_0 = execution_core::state::encode_checkpoint(&checkpoint_state).unwrap();
    std::fs::write(&checkpoint_path, &checkpoint_0).unwrap();
    let state0 = checkpoint_state.root();

    let replay_root_0 = decode_state_replay_root_bytes(
        checkpoint_state
            .entries
            .get(REPLAY_ROOT_STATE_KEY.as_bytes())
            .expect("genesis replay root exists"),
    );

    let i1 = VmExecutionInput {
        package_manifest_root: package_root,
        civilization_root: package_root,
        pre_state_root: state0,
        prior_replay_root_value: replay_root_0,
        checkpoint_root: h(31),
        payload_root: h(31),
    };
    let (r1, _) = execute_vm_boundary(&i1);
    let mut state1 = checkpoint_state.clone();
    let replay_root_before = checkpoint_state
        .entries
        .get(REPLAY_ROOT_STATE_KEY.as_bytes())
        .cloned()
        .unwrap();
    assert_eq!(
        r1.state_diff[0].before.as_bytes().to_vec(),
        replay_root_before
    );
    execution_core::state::apply_diff(&mut state1, &r1.state_diff).unwrap();
    let state1_root = state1.root();
    let replay_root_1 = decode_state_replay_root_bytes(
        state1
            .entries
            .get(REPLAY_ROOT_STATE_KEY.as_bytes())
            .expect("replay root exists after first execution"),
    );
    let i2 = VmExecutionInput {
        package_manifest_root: package_root,
        civilization_root: package_root,
        pre_state_root: state1_root,
        prior_replay_root_value: replay_root_1,
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
    assert!(matches!(err, ChainRestoreError::Validation(ref m) if m.field == "checkpoint_root" || m.field == "checkpoint_decode"));
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
    assert!(matches!(err, ChainRestoreError::Validation(ref m) if m.field == "execution_id") || matches!(err, ChainRestoreError::Lineage(_)));
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

#[test]
fn test_fixture_no_synthetic_replay_root_seed() {
    let (_t, input, _lineage) = fixture_two_step();
    let state0: execution_core::state::CanonicalState =
        execution_core::state::decode_checkpoint(&std::fs::read(&input.checkpoint_path).unwrap())
            .unwrap();
    let synthetic = "0707070707070707070707070707070707070707070707070707070707070707";
    assert!(state0.entries.values().all(|v| v != synthetic.as_bytes()));

    for receipt_path in &input.receipt_paths {
        let receipt: execution_core::vm::VmExecutionReceipt =
            bincode::deserialize(&std::fs::read(receipt_path).unwrap()).unwrap();
        for change in &receipt.state_diff {
            if change.key == "__replay_root__" {
                assert_ne!(change.before, synthetic);
            }
        }
    }
}

#[test]
fn test_genesis_replay_root_present_in_checkpoint_0() {
    let (_t, input, _lineage) = fixture_two_step();
    let state0: execution_core::state::CanonicalState =
        execution_core::state::decode_checkpoint(&std::fs::read(&input.checkpoint_path).unwrap())
            .unwrap();
    let expected = genesis_replay_root_value();
    assert_eq!(
        state0.entries.get(REPLAY_ROOT_STATE_KEY.as_bytes()),
        Some(&expected)
    );

    let r1: execution_core::vm::VmExecutionReceipt =
        bincode::deserialize(&std::fs::read(&input.receipt_paths[0]).unwrap()).unwrap();
    let replay_root_before = r1
        .state_diff
        .iter()
        .find(|change| change.key == REPLAY_ROOT_STATE_KEY)
        .map(|change| change.before.as_bytes().to_vec())
        .unwrap();
    assert_eq!(replay_root_before, expected);
}

#[test]
fn test_chain_restore_replay_root_semantic_layers() {
    let (_t, input, lineage) = fixture_two_step();
    let state0: execution_core::state::CanonicalState =
        execution_core::state::decode_checkpoint(&std::fs::read(&input.checkpoint_path).unwrap())
            .unwrap();
    let pre_state_root = state0.root();
    assert_eq!(pre_state_root, lineage.records[0].pre_state_root);

    let prior_replay_root_value = decode_state_replay_root_bytes(
        state0
            .entries
            .get(REPLAY_ROOT_STATE_KEY.as_bytes())
            .expect("genesis replay root exists"),
    );

    let r1: execution_core::vm::VmExecutionReceipt =
        bincode::deserialize(&std::fs::read(&input.receipt_paths[0]).unwrap()).unwrap();
    let replay_root_change = r1
        .state_diff
        .iter()
        .find(|change| change.key == REPLAY_ROOT_STATE_KEY)
        .expect("replay root diff exists");

    assert_eq!(r1.prior_replay_root, pre_state_root);
    assert_eq!(
        replay_root_change.before.as_bytes(),
        hex::encode(prior_replay_root_value).as_bytes()
    );
    assert_eq!(
        replay_root_change.after.as_bytes(),
        hex::encode(r1.next_replay_root).as_bytes()
    );
}
