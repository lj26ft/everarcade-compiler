use serde::{Deserialize, Serialize};

use crate::federation::node::FederationNodeId;

use super::{epoch::AuthorityEpoch, errors::AuthorityError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityRegistry {
    pub current_authority: FederationNodeId,
    pub current_epoch: u64,
}

pub fn update_authority_registry(
    registry: &AuthorityRegistry,
    next_epoch: &AuthorityEpoch,
) -> Result<AuthorityRegistry, AuthorityError> {
    if next_epoch.epoch <= registry.current_epoch {
        return Err(AuthorityError::EpochRollback);
    }
    Ok(AuthorityRegistry {
        current_authority: next_epoch.authority,
        current_epoch: next_epoch.epoch,
    })
}
