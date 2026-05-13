use execution_core::jurisprudence::legal_lineage::legal_lineage_root;

#[test]
fn jurisprudence_lineage_deterministic() {
    assert_eq!(legal_lineage_root([1; 32], [2; 32]), legal_lineage_root([1; 32], [2; 32]));
}
