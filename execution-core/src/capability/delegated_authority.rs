use super::capability::Capability;

pub fn is_explicit_delegation(capability: &Capability) -> bool {
    capability.parent_capability.is_some()
}
