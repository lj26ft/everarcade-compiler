use everarcade_host::{
    replay_sync::replay_validation::replay_roots_match,
    security::security_validation::security_decision,
};
#[test]
fn replay_truth_preserved_over_signature() {
    let replay_ok = replay_roots_match([1; 32], [2; 32]);
    let signed = true;
    assert!(!security_decision(replay_ok, signed));
}
