use everarcade_host::convergence::replay_compare::compare_replay_roots;
#[test]
fn multi_node_convergence() {
    assert!(compare_replay_roots([5; 32], [5; 32]));
}
