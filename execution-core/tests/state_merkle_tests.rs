use execution_core::{execution::ExecutionState, merkle::state_merkle::state_root};

#[test]
fn state_root_stable_for_same_entries() {
    let mut a = ExecutionState::default();
    a.values.insert("x".into(), vec![1, 2]);
    a.values.insert("y".into(), vec![3]);

    let mut b = ExecutionState::default();
    b.values.insert("y".into(), vec![3]);
    b.values.insert("x".into(), vec![1, 2]);

    assert_eq!(state_root(&a), state_root(&b));
}
