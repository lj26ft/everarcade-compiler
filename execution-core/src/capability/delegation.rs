use super::capability::Capability;

pub fn delegate(parent: &Capability, delegated_capability_id: [u8; 32]) -> Capability {
    Capability {
        capability_id: delegated_capability_id,
        issuing_domain: parent.issuing_domain,
        authority_scope: parent.authority_scope,
        parent_capability: Some(parent.capability_id),
        revocation_root: None,
    }
}
