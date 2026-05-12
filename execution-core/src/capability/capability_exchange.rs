use super::capability::Capability;

pub fn exchange_capability(capability: &Capability, receiving_domain: [u8; 32], treaty_root: [u8; 32]) -> Capability {
    Capability { capability_id: capability.capability_id, issuing_domain: receiving_domain, authority_scope: treaty_root, parent_capability: Some(capability.capability_id), revocation_root: capability.revocation_root }
}
