use execution_core::economy::ledger::{ledger_root, EconomicLedgerEntry};
#[test]
fn ledger_determinism() {
    let e = EconomicLedgerEntry::new(
        1,
        "asset".into(),
        "alice".into(),
        "bob".into(),
        "vault".into(),
        "settle".into(),
    );
    let r1 = ledger_root(&[e.clone()]).unwrap();
    let r2 = ledger_root(&[e]).unwrap();
    assert_eq!(r1, r2);
}
