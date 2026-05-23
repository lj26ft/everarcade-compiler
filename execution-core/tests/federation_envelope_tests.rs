use execution_core::federation::envelope::FederationContinuityEnvelope;
#[test]
fn envelope_determinism() {
    let e = FederationContinuityEnvelope {
        version: 1,
        execution_manifest_hash: "m".into(),
        checkpoint_hash: "c".into(),
        settlement_root: "s".into(),
        replay_root: "r".into(),
        node_identity: "n".into(),
        continuity_proof_hash: "p".into(),
        detached_signature: None,
    };
    assert_eq!(e.canonical_hash().unwrap(), e.canonical_hash().unwrap());
}
