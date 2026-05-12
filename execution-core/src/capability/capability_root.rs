use super::capability::{Capability, Hash};

pub fn capability_root(capability: &Capability) -> Hash {
    capability.capability_id
}
