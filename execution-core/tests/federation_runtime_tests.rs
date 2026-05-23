use execution_core::federation::{
    continuity::FederationContinuityProof, manifest::FederationExecutionManifest,
    node_identity::NodeIdentityContinuity,
};
#[test]
fn runtime_primitives_deterministic() {
    let m = FederationExecutionManifest {
        federation_epoch: 1,
        execution_manifest_hash: "e".into(),
        checkpoint_hash: "c".into(),
        settlement_hash: "s".into(),
        quorum_hash: "q".into(),
        participating_nodes: vec!["n2".into(), "n1".into()],
        state_root: "sr".into(),
        replay_root: "rr".into(),
    };
    assert_eq!(
        m.canonical_hash().unwrap(),
        m.clone().canonicalized().canonical_hash().unwrap()
    );
    let p = FederationContinuityProof::new("a".into(), "b".into(), "c".into());
    assert!(p.verify().is_ok());
    let i = NodeIdentityContinuity::new("n".into(), "prev".into(), "cp".into());
    assert_eq!(
        i.identity_hash,
        NodeIdentityContinuity::new("n".into(), "prev".into(), "cp".into()).identity_hash
    );
}
