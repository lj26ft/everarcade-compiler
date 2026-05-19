use execution_core::runtime_commit::*;

fn sample_change(k: &[u8]) -> StateChange {
    StateChange {
        key: k.to_vec(),
        before: vec![0],
        after: vec![1],
    }
}

#[test]
fn test_state_diff_hash_is_deterministic() {
    let input = CommitInput {
        contract_id: "c".into(),
        execution_id: "e".into(),
        previous_state_root: [1; 32],
        state_changes: vec![sample_change(b"a")],
        fuel_used: 1,
        previous_entry_hash: [0; 32],
        expected_sequence_number: 0,
        is_noop: false,
    };
    let a = commit_execution(input.clone()).unwrap();
    let b = commit_execution(input).unwrap();
    assert_eq!(a.state_diff, b.state_diff);
}
#[test]
fn test_state_changes_are_sorted() {
    let c = canonicalize_changes(vec![sample_change(b"b"), sample_change(b"a")], false).unwrap();
    assert_eq!(c[0].key, b"a");
}
#[test]
fn test_duplicate_state_keys_rejected() {
    assert!(canonicalize_changes(vec![sample_change(b"a"), sample_change(b"a")], false).is_err());
}
#[test]
fn test_receipt_hash_excludes_self() {
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
    assert_eq!(out.receipt.receipt_hash, out.receipt.immutable_hash());
}
#[test]
fn test_receipt_binds_previous_and_new_state_root() {
    let out = commit_execution(CommitInput {
        contract_id: "c".into(),
        execution_id: "e".into(),
        previous_state_root: [9; 32],
        state_changes: vec![sample_change(b"a")],
        fuel_used: 1,
        previous_entry_hash: [0; 32],
        expected_sequence_number: 0,
        is_noop: false,
    })
    .unwrap();
    assert_eq!(out.receipt.previous_state_root, [9; 32]);
    assert_eq!(out.receipt.new_state_root, out.state_diff.new_state_root);
}
#[test]
fn test_journal_entry_hash_chain_is_deterministic() {
    let a = commit_execution(CommitInput {
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
    let b = commit_execution(CommitInput {
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
    assert_eq!(a.journal_entry.entry_hash, b.journal_entry.entry_hash);
}
#[test]
fn test_journal_rejects_previous_hash_mismatch() {
    let out = commit_execution(CommitInput {
        contract_id: "c".into(),
        execution_id: "e".into(),
        previous_state_root: [1; 32],
        state_changes: vec![sample_change(b"a")],
        fuel_used: 1,
        previous_entry_hash: [5; 32],
        expected_sequence_number: 0,
        is_noop: false,
    })
    .unwrap();
    assert!(replay_verify(&[out]).is_err());
}
#[test]
fn test_checkpoint_binds_state_root_to_journal() {
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
    assert_eq!(out.checkpoint.state_root, out.state_diff.new_state_root);
    assert_eq!(
        out.checkpoint.journal_entry_hash,
        out.journal_entry.entry_hash
    );
}
#[test]
fn test_commit_pipeline_is_deterministic() {
    let i = CommitInput {
        contract_id: "c".into(),
        execution_id: "e".into(),
        previous_state_root: [1; 32],
        state_changes: vec![sample_change(b"a")],
        fuel_used: 1,
        previous_entry_hash: [0; 32],
        expected_sequence_number: 0,
        is_noop: false,
    };
    assert_eq!(
        commit_execution(i.clone())
            .unwrap()
            .journal_entry
            .entry_hash,
        commit_execution(i).unwrap().journal_entry.entry_hash
    );
}
#[test]
fn test_replay_reconstructs_same_state_root() {
    let a = commit_execution(CommitInput {
        contract_id: "c".into(),
        execution_id: "e1".into(),
        previous_state_root: [1; 32],
        state_changes: vec![sample_change(b"a")],
        fuel_used: 1,
        previous_entry_hash: [0; 32],
        expected_sequence_number: 0,
        is_noop: false,
    })
    .unwrap();
    let b = commit_execution(CommitInput {
        contract_id: "c".into(),
        execution_id: "e2".into(),
        previous_state_root: a.state_diff.new_state_root,
        state_changes: vec![sample_change(b"b")],
        fuel_used: 1,
        previous_entry_hash: a.journal_entry.entry_hash,
        expected_sequence_number: 1,
        is_noop: false,
    })
    .unwrap();
    let r = replay_verify(&[a, b.clone()]).unwrap();
    assert_eq!(r.reconstructed_state_root, b.state_diff.new_state_root);
}
