use execution_core::federation::quorum::FederationQuorumProof;
#[test]
fn quorum_determinism() {
    let a = FederationQuorumProof::new(
        "r".into(),
        "s".into(),
        "rp".into(),
        vec!["b".into(), "a".into()],
    );
    let b = FederationQuorumProof::new(
        "r".into(),
        "s".into(),
        "rp".into(),
        vec!["a".into(), "b".into()],
    );
    assert_eq!(a.quorum_hash, b.quorum_hash);
}
