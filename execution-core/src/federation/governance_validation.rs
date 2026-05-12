use super::governance_capability::GovernanceCapability;

pub fn validate_governance_capability(capability: &GovernanceCapability) -> bool {
    capability.capability_id != [0; 32] && capability.constitutional_scope != [0; 32]
}
