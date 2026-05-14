use super::capability::Capability;

pub fn is_external(capability: &Capability, local_domain: [u8; 32]) -> bool {
    capability.issuing_domain != local_domain
}
