use execution_core::economy::{
    ledger::{ledger_root, EconomicLedgerEntry},
    xrpl_anchor::XrplSettlementAnchor,
};
#[test]
fn world_persistence_integration_smoke() {
    let entry = EconomicLedgerEntry::new(
        1,
        "asset".into(),
        "alice".into(),
        "bob".into(),
        "vault".into(),
        "settle".into(),
    );
    let root = ledger_root(&[entry]).unwrap();
    let anchor = XrplSettlementAnchor {
        settlement_commitment: root.clone(),
        replay_root: "r".into(),
        checkpoint_root: "c".into(),
        ownership_root: "o".into(),
    };
    assert_eq!(anchor.settlement_commitment, root);
}
