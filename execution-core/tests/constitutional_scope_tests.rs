use execution_core::federation::{constitutional_scope::constitutional_scope, governance_capability::GovernanceCapability};

#[test]
fn constitutional_scope_is_explicit() {
    let capability = GovernanceCapability { capability_id: [1; 32], constitutional_scope: [8; 32] };
    assert_eq!(constitutional_scope(&capability), [8; 32]);
}
