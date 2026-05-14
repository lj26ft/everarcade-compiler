use everarcade_host::convergence::divergence_detection::reject_on_divergence;

#[test]
fn mismatched_replay_lineage_is_rejected() {
    assert!(reject_on_divergence([1; 32], [2; 32]));
}
