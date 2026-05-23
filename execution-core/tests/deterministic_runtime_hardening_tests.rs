use execution_core::journal::ExecutionJournal;
use execution_core::receipt_canonical::CanonicalExecutionReceipt;
use execution_core::state::CanonicalExecutionState;
use execution_core::state_root::canonical_state_root;
use std::collections::BTreeMap;

#[test]
fn state_serialization_is_deterministic() {
    let mut entries = BTreeMap::new();
    entries.insert("b".to_string(), vec![2]);
    entries.insert("a".to_string(), vec![1]);
    let state = CanonicalExecutionState {
        revision: 7,
        entries,
    };

    assert_eq!(state.to_canonical_bytes(), state.to_canonical_bytes());
    assert_eq!(state.to_canonical_json(), state.to_canonical_json());
    assert_eq!(state.canonical_hash(), state.canonical_hash());
}

#[test]
fn state_root_is_order_independent() {
    let mut a = BTreeMap::new();
    a.insert("counter".to_string(), vec![1]);
    a.insert("level".to_string(), vec![9]);

    let mut b = BTreeMap::new();
    b.insert("level".to_string(), vec![9]);
    b.insert("counter".to_string(), vec![1]);

    assert_eq!(canonical_state_root(&a), canonical_state_root(&b));
}

#[test]
fn journal_hash_is_replay_stable() {
    let receipt = CanonicalExecutionReceipt {
        execution_id: [1; 32],
        module_hash: [2; 32],
        input_hash: [3; 32],
        fuel_consumed: 42,
        state_root_before: [4; 32],
        state_root_after: [5; 32],
        state_diff_hash: [6; 32],
        replay_hash: [7; 32],
        exit_code: 0,
    };
    let mut j1 = ExecutionJournal::default();
    j1.append([8; 32], &receipt);
    j1.append([9; 32], &receipt);

    let mut j2 = ExecutionJournal::default();
    j2.append([8; 32], &receipt);
    j2.append([9; 32], &receipt);

    assert_eq!(j1, j2);
    assert_eq!(j1.canonical_hash(), j2.canonical_hash());
}
