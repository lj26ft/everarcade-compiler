use super::capability::{Capability, Hash};

pub fn authority_scope(capability: &Capability) -> Hash {
    capability.authority_scope
}
