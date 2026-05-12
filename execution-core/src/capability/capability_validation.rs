use super::capability::Capability;

pub fn validate_capability(capability: &Capability) -> bool {
    capability.capability_id != [0; 32]
        && capability.issuing_domain != [0; 32]
        && capability.authority_scope != [0; 32]
}
