use execution_core::hashing::hash_bytes;
use execution_core::replay::replay_receipt_chain;

#[test]
fn replay_detects_parent_break() {
    let prior = hash_bytes(b"genesis");
    let diff = hash_bytes(b"r1");
    let root1 = hash_bytes([prior.as_bytes(), diff.as_bytes()].concat().as_slice());
    let out = replay_receipt_chain(&[("r1", None, &root1), ("r2", Some("x"), "")]);
    assert_eq!(out.divergence, Some(1));
}
