use super::governance_capability::GovernanceCapability;

pub fn delegate_federated(capability: &GovernanceCapability, capability_id: [u8; 32]) -> GovernanceCapability {
    GovernanceCapability {
        capability_id,
        constitutional_scope: capability.constitutional_scope,
    }
}
