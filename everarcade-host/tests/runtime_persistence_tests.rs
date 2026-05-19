use std::fs;

use everarcade_host::runtime_persistence::{
    persist_commit_records, replay_world, verify_journal, verify_world,
};
use execution_core::runtime_commit::{commit_execution, CommitInput, StateChange};

fn sample_change(k: &[u8]) -> StateChange {
    StateChange {
        key: k.to_vec(),
        before: vec![0],
        after: vec![1],
    }
}

#[test]
fn test_persist_receipt_to_world_root() {
    let d = tempfile::tempdir().unwrap();
    let out = commit_execution(CommitInput {
        contract_id: "c".into(),
        execution_id: "e".into(),
        previous_state_root: [1; 32],
        state_changes: vec![sample_change(b"a")],
        fuel_used: 1,
        previous_entry_hash: [0; 32],
        expected_sequence_number: 0,
        is_noop: false,
    })
    .unwrap();
    persist_commit_records(d.path(), &out.receipt, &out.journal_entry, &out.checkpoint).unwrap();
    assert!(d.path().join("receipts").exists());
}
#[test]
fn test_persist_journal_entry_append_only() {
    let d = tempfile::tempdir().unwrap();
    let out = commit_execution(CommitInput {
        contract_id: "c".into(),
        execution_id: "e".into(),
        previous_state_root: [1; 32],
        state_changes: vec![sample_change(b"a")],
        fuel_used: 1,
        previous_entry_hash: [0; 32],
        expected_sequence_number: 0,
        is_noop: false,
    })
    .unwrap();
    persist_commit_records(d.path(), &out.receipt, &out.journal_entry, &out.checkpoint).unwrap();
    assert!(verify_journal(d.path()).is_ok());
}
#[test]
fn test_persist_checkpoint_record() {
    let d = tempfile::tempdir().unwrap();
    let out = commit_execution(CommitInput {
        contract_id: "c".into(),
        execution_id: "e".into(),
        previous_state_root: [1; 32],
        state_changes: vec![sample_change(b"a")],
        fuel_used: 1,
        previous_entry_hash: [0; 32],
        expected_sequence_number: 0,
        is_noop: false,
    })
    .unwrap();
    persist_commit_records(d.path(), &out.receipt, &out.journal_entry, &out.checkpoint).unwrap();
    assert!(d.path().join("checkpoints").exists());
}
#[test]
fn test_verify_world_detects_missing_receipt() {
    let d = tempfile::tempdir().unwrap();
    let out = commit_execution(CommitInput {
        contract_id: "c".into(),
        execution_id: "e".into(),
        previous_state_root: [1; 32],
        state_changes: vec![sample_change(b"a")],
        fuel_used: 1,
        previous_entry_hash: [0; 32],
        expected_sequence_number: 0,
        is_noop: false,
    })
    .unwrap();
    persist_commit_records(d.path(), &out.receipt, &out.journal_entry, &out.checkpoint).unwrap();
    fs::remove_dir_all(d.path().join("receipts")).unwrap();
    assert!(verify_world(d.path()).is_err());
}
#[test]
fn test_verify_world_detects_broken_journal_chain() {
    let d = tempfile::tempdir().unwrap();
    let mut out = commit_execution(CommitInput {
        contract_id: "c".into(),
        execution_id: "e".into(),
        previous_state_root: [1; 32],
        state_changes: vec![sample_change(b"a")],
        fuel_used: 1,
        previous_entry_hash: [0; 32],
        expected_sequence_number: 0,
        is_noop: false,
    })
    .unwrap();
    out.journal_entry.previous_entry_hash = [1; 32];
    persist_commit_records(d.path(), &out.receipt, &out.journal_entry, &out.checkpoint).unwrap();
    assert!(verify_world(d.path()).is_err());
}
#[test]
fn test_replay_world_reconstructs_latest_checkpoint() {
    let d = tempfile::tempdir().unwrap();
    let out = commit_execution(CommitInput {
        contract_id: "c".into(),
        execution_id: "e".into(),
        previous_state_root: [1; 32],
        state_changes: vec![sample_change(b"a")],
        fuel_used: 1,
        previous_entry_hash: [0; 32],
        expected_sequence_number: 0,
        is_noop: false,
    })
    .unwrap();
    let root = out.state_diff.new_state_root;
    persist_commit_records(d.path(), &out.receipt, &out.journal_entry, &out.checkpoint).unwrap();
    assert_eq!(replay_world(d.path()).unwrap(), root);
}
