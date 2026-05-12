use execution_core::capability::{capability::Capability, capability_lineage::inherits_from};

#[test]
fn capability_lineage_is_preserved() {
    let parent = Capability { capability_id: [1; 32], issuing_domain: [2; 32], authority_scope: [3; 32], parent_capability: None, revocation_root: None };
    let child = Capability { capability_id: [4; 32], issuing_domain: [2; 32], authority_scope: [3; 32], parent_capability: Some([1; 32]), revocation_root: None };
    assert!(inherits_from(&child, &parent));
}
