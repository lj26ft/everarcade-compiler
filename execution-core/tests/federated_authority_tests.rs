use execution_core::federation::{governance_capability::GovernanceCapability, governance_validation::validate_governance_capability};

#[test]
fn federated_authority_validation_rejects_zero_roots() {
    let invalid = GovernanceCapability { capability_id: [0; 32], constitutional_scope: [1; 32] };
    let valid = GovernanceCapability { capability_id: [1; 32], constitutional_scope: [2; 32] };
    assert!(!validate_governance_capability(&invalid));
    assert!(validate_governance_capability(&valid));
}
