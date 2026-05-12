use super::governance_capability::GovernanceCapability;

pub fn constitutional_scope(capability: &GovernanceCapability) -> [u8; 32] {
    capability.constitutional_scope
}
