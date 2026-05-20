use super::{checkpoint::GovernanceCheckpoint, error::GovernanceError};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernancePolicy {
    pub epoch: u64,
    pub federation_participation: bool,
    pub runtime_permissions: Vec<String>,
    pub checkpoint: [u8; 32],
    pub policy_id: [u8; 32],
}
pub fn apply_governance_policy(
    epoch: u64,
    federation_participation: bool,
    runtime_permissions: Vec<String>,
    checkpoint: &GovernanceCheckpoint,
) -> GovernancePolicy {
    let mut perms = runtime_permissions;
    perms.sort();
    let mut h = Sha256::new();
    h.update(epoch.to_le_bytes());
    h.update([federation_participation as u8]);
    for p in &perms {
        h.update(p.as_bytes())
    }
    h.update(checkpoint.policy_root);
    GovernancePolicy {
        epoch,
        federation_participation,
        runtime_permissions: perms,
        checkpoint: checkpoint.policy_root,
        policy_id: h.finalize().into(),
    }
}
pub fn verify_policy_continuity(policies: &[GovernancePolicy]) -> Result<(), GovernanceError> {
    for w in policies.windows(2) {
        if w[1].epoch <= w[0].epoch || w[1].checkpoint == [0; 32] {
            return Err(GovernanceError::PolicyContinuity);
        }
    }
    Ok(())
}
