use execution_core::capability::capability::Capability;
#[test]
fn revocation_propagation_is_explicit() {
    let cap = Capability {
        capability_id: [1; 32],
        issuing_domain: [2; 32],
        authority_scope: [3; 32],
        parent_capability: Some([0; 32]),
        revocation_root: Some([9; 32]),
    };
    assert_eq!(cap.revocation_root, Some([9; 32]));
}
