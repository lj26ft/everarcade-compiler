use execution_core::federation::replay_verify::{
    verify_replay_equivalence, ReplayVerificationInput,
};
#[test]
fn replay_equivalence_and_divergence() {
    let a = ReplayVerificationInput {
        receipt_hash: "a".into(),
        state_root: "b".into(),
        replay_root: "c".into(),
        settlement_root: "d".into(),
    };
    assert!(verify_replay_equivalence(&a, &a).is_ok());
    let mut b = a.clone();
    b.state_root = "x".into();
    assert!(verify_replay_equivalence(&a, &b).is_err());
}
