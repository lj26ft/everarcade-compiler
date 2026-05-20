use std::fs;

use execution_core::runtime_commit::{commit_execution, CommitInput, StateChange};

#[test]
fn test_replay_reconstructs_state_root() {
    let tmp = tempfile::tempdir().unwrap();
    let world = tmp.path();
    fs::create_dir_all(world.join("receipts")).unwrap();
    fs::create_dir_all(world.join("journal")).unwrap();
    fs::create_dir_all(world.join("checkpoints")).unwrap();

    let out = commit_execution(CommitInput {
        contract_id: "counter".into(),
        execution_id: "1".into(),
        previous_state_root: [0u8; 32],
        state_changes: vec![StateChange {
            key: b"counter".to_vec(),
            before: vec![0],
            after: vec![1],
        }],
        fuel_used: 7,
        previous_entry_hash: [0u8; 32],
        expected_sequence_number: 0,
        is_noop: false,
    })
    .unwrap();

    everarcade_host::runtime_persistence::persist_commit_records(
        world,
        &out.receipt,
        &out.journal_entry,
        &out.checkpoint,
    )
    .unwrap();
    let reconstructed = everarcade_host::runtime_replay::replay_world(world).unwrap();
    assert_eq!(reconstructed, out.receipt.new_state_root);
}

#[test]
fn test_verify_world_detects_missing_receipt() {
    let tmp = tempfile::tempdir().unwrap();
    let world = tmp.path();
    fs::create_dir_all(world.join("receipts")).unwrap();
    fs::create_dir_all(world.join("journal")).unwrap();
    fs::create_dir_all(world.join("checkpoints")).unwrap();

    let out = commit_execution(CommitInput {
        contract_id: "counter".into(),
        execution_id: "1".into(),
        previous_state_root: [0u8; 32],
        state_changes: vec![StateChange {
            key: b"counter".to_vec(),
            before: vec![0],
            after: vec![1],
        }],
        fuel_used: 7,
        previous_entry_hash: [0u8; 32],
        expected_sequence_number: 0,
        is_noop: false,
    })
    .unwrap();
    fs::write(
        world.join("journal/00000000000000000000.json"),
        serde_json::to_vec_pretty(&out.journal_entry).unwrap(),
    )
    .unwrap();
    fs::write(
        world.join("checkpoints/00000000000000000000.json"),
        serde_json::to_vec_pretty(&out.checkpoint).unwrap(),
    )
    .unwrap();

    let err = everarcade_host::runtime_replay::verify_world(world)
        .unwrap_err()
        .to_string();
    assert!(err.contains("missing receipt"));
}
