use super::capability::Capability;

pub fn is_treaty_scoped(capability: &Capability, treaty_root: [u8; 32]) -> bool { capability.authority_scope == treaty_root }
