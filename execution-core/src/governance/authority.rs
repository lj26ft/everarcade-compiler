use super::error::GovernanceError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthorityState {
    pub authority: [u8; 32],
    pub epoch: u64,
    pub previous: Option<[u8; 32]>,
    pub permissions: Vec<String>,
    pub state_id: [u8; 32],
}
pub fn assign_runtime_authority(
    authority: [u8; 32],
    epoch: u64,
    permissions: Vec<String>,
) -> AuthorityState {
    let mut h = Sha256::new();
    h.update(authority);
    h.update(epoch.to_le_bytes());
    for p in &permissions {
        h.update(p.as_bytes())
    }
    AuthorityState {
        authority,
        epoch,
        previous: None,
        permissions,
        state_id: h.finalize().into(),
    }
}
pub fn transfer_runtime_authority(
    prev: &AuthorityState,
    to: [u8; 32],
    permissions: Vec<String>,
) -> AuthorityState {
    let mut next = assign_runtime_authority(to, prev.epoch + 1, permissions);
    next.previous = Some(prev.state_id);
    next
}
pub fn verify_authority_lineage(states: &[AuthorityState]) -> Result<(), GovernanceError> {
    for w in states.windows(2) {
        if w[1].previous != Some(w[0].state_id) || w[1].epoch != w[0].epoch + 1 {
            return Err(GovernanceError::AuthorityContinuity);
        }
    }
    Ok(())
}
