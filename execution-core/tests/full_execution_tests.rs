use execution_core::runtime_commit::{commit_execution, CommitInput, StateChange};

#[test]
fn test_replay_reproduces_execution_hashes() {
    let input = CommitInput {
        contract_id: "counter".into(),
        execution_id: "exec-1".into(),
        previous_state_root: [0u8; 32],
        state_changes: vec![StateChange {
            key: b"counter".to_vec(),
            before: vec![0],
            after: vec![1],
        }],
        fuel_used: 11,
        previous_entry_hash: [0u8; 32],
        expected_sequence_number: 0,
        is_noop: false,
    };
    let a = commit_execution(input.clone()).unwrap();
    let b = commit_execution(input).unwrap();
    assert_eq!(a.receipt.receipt_hash, b.receipt.receipt_hash);
    assert_eq!(a.journal_entry.entry_hash, b.journal_entry.entry_hash);
    assert_eq!(a.checkpoint.checkpoint_hash, b.checkpoint.checkpoint_hash);
}
