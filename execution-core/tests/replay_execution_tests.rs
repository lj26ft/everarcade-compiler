use execution_core::replay::replay_receipt_chain;

#[test]
fn replay_detects_divergence_on_root_mismatch() {
    let out = replay_receipt_chain(&[("r1", None, "bad_root")]);
    assert_eq!(out.divergence, Some(0));
}
