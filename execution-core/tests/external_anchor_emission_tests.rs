use execution_core::external::anchor_emission::emit_external_anchor_receipt;
use execution_core::external::anchor_validation::validate_external_anchor_receipt;

#[test]
fn external_anchor_emission_is_deterministic() {
    let a = emit_external_anchor_receipt([9; 32]);
    let b = emit_external_anchor_receipt([9; 32]);
    assert_eq!(a, b);
    assert!(validate_external_anchor_receipt(&a));
}
