use everarcade_host::reconciliation::execution_reconciliation::select_highest_valid_replay_continuity;

#[test]
fn highest_valid_replay_continuity_selected() {
    let selected = select_highest_valid_replay_continuity(&[[1; 32], [2; 32], [3; 32]]).unwrap();
    assert_eq!(selected, [3; 32]);
}
