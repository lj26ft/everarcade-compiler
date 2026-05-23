use execution_core::federation::snapshot::FederationSnapshot;
#[test]
fn snapshot_eq() {
    let s = FederationSnapshot {
        state_root: "s".into(),
        replay_root: "r".into(),
        checkpoint_root: "c".into(),
        settlement_root: "st".into(),
        quorum_root: "q".into(),
        federation_manifest_hash: "m".into(),
    };
    assert_eq!(s.canonical_hash().unwrap(), s.canonical_hash().unwrap());
}
