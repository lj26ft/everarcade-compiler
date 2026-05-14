use execution_core::monetary::issuance_lineage::derive_issuance_lineage;
#[test]
fn monetary_lineage_continuity() {
    let p = [1u8; 32];
    let i = [2u8; 32];
    assert_eq!(derive_issuance_lineage(p, i), derive_issuance_lineage(p, i));
}
