use everarcade_host::reconciliation::execution_continuity_validation::highest_valid_replay;

#[test]
fn highest_valid_replay_selected() {
    assert_eq!(highest_valid_replay(10, 12, true), 12);
    assert_eq!(highest_valid_replay(10, 12, false), 10);
}
