use execution_core::interpretation::{
    interpretation::ConstitutionalInterpretation, interpretation_root::interpretation_root,
};

#[test]
fn interpretation_root_determinism() {
    let i = ConstitutionalInterpretation {
        interpretation_id: [1; 32],
        constitutional_root: [2; 32],
        interpretation_scope_root: [3; 32],
        lineage_root: [4; 32],
    };
    assert_eq!(interpretation_root(&[i.clone()]), interpretation_root(&[i]));
}
