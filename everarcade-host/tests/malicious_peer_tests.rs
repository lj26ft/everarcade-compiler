use everarcade_host::security::{
    divergence_detection::divergence_detected, malformed_replay::malformed_replay_detected,
};
#[test]
fn malformed_and_divergence_detected() {
    assert!(malformed_replay_detected([0; 32]));
    assert!(divergence_detected([1; 32], [2; 32]));
}
