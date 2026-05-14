use execution_core::capability::{capability::Capability, delegation::delegate};

#[test]
fn delegation_keeps_domain_and_scope() {
    let cap = Capability {
        capability_id: [1; 32],
        issuing_domain: [2; 32],
        authority_scope: [3; 32],
        parent_capability: None,
        revocation_root: None,
    };
    let delegated = delegate(&cap, [7; 32]);
    assert_eq!(delegated.issuing_domain, [2; 32]);
    assert_eq!(delegated.authority_scope, [3; 32]);
    assert_eq!(delegated.parent_capability, Some([1; 32]));
}
