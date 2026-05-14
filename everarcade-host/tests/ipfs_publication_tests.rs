use everarcade_host::ipfs::{publisher::build_intent, validation::validate_intent};
#[test]
fn builds_ipfs_intent() {
    let i = build_intent([1; 32], "state/artifact.bin".into(), b"abc");
    assert!(validate_intent(&i));
    assert!(i.cid.is_some());
}
