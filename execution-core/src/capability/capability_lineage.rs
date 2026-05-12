use super::capability::Capability;

pub fn inherits_from(child: &Capability, parent: &Capability) -> bool {
    child.parent_capability == Some(parent.capability_id)
}
