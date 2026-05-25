use execution_core::security::*;

#[test]
fn security_validation_root_is_deterministic() {
    let c = CapabilityValidationRoot("a".into());
    let g = GovernanceValidationRoot("b".into());
    let i = IsolationValidationRoot("c".into());
    assert_eq!(
        SecurityValidationRoot::derive(&c, &g, &i),
        SecurityValidationRoot::derive(&c, &g, &i)
    );
}
