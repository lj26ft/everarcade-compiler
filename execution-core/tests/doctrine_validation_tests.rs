use execution_core::jurisprudence::{doctrine_validation::validate_doctrine, precedent::LegalPrecedent};

#[test]
fn doctrine_validation() {
    let precedent = LegalPrecedent { precedent_id:[1;32], constitutional_root:[2;32], interpretation_root:[3;32], lineage_root:[4;32] };
    assert!(validate_doctrine(&precedent));
}
