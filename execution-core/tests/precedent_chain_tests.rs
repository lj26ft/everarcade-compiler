use execution_core::jurisprudence::{
    precedent::LegalPrecedent, precedent_chain::precedent_chain_is_continuous,
};

#[test]
fn precedent_stability() {
    let a = LegalPrecedent {
        precedent_id: [1; 32],
        constitutional_root: [2; 32],
        interpretation_root: [3; 32],
        lineage_root: [4; 32],
    };
    let b = LegalPrecedent {
        precedent_id: [5; 32],
        constitutional_root: [2; 32],
        interpretation_root: [3; 32],
        lineage_root: [6; 32],
    };
    assert!(precedent_chain_is_continuous(&[a, b]));
}
