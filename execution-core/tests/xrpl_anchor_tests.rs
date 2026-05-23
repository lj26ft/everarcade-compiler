use execution_core::economy::xrpl_anchor::XrplSettlementAnchor;
#[test]
fn xrpl_anchor_determinism() {
    let x = XrplSettlementAnchor {
        settlement_commitment: "s".into(),
        replay_root: "r".into(),
        checkpoint_root: "c".into(),
        ownership_root: "o".into(),
    };
    assert!(x.verify("r", "c", "o").is_ok());
    assert_eq!(x.canonical_hash().unwrap(), x.canonical_hash().unwrap());
}
