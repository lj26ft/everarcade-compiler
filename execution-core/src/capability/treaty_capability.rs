use super::capability::Capability;

pub fn bind_to_treaty(mut capability: Capability, treaty_root: [u8; 32]) -> Capability {
    capability.authority_scope = treaty_root;
    capability
}
