use execution_core::capability::{capability::Capability, revocation::revoke};

#[test]
fn capability_revocation_sets_root() {
    let cap = Capability { capability_id: [1; 32], issuing_domain: [2; 32], authority_scope: [3; 32], parent_capability: None, revocation_root: None };
    let revoked = revoke(&cap, [9; 32]);
    assert_eq!(revoked.revocation_root, Some([9; 32]));
}
