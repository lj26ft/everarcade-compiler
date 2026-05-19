use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{canonical::encoding::canonical_encode, operator::continuity::Hash256};

use super::{errors::CoordinationError, registry::CoordinationRegistry};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationState {
    pub registry_hash: Hash256,
    pub active_session_count: usize,
}

pub fn hash_coordination_state(state: &CoordinationState) -> Hash256 {
    Sha256::digest(&canonical_encode(state).expect("coordination state encode")).into()
}

pub fn verify_coordination_state(
    state: &CoordinationState,
    registry: &CoordinationRegistry,
) -> Result<(), CoordinationError> {
    if state.registry_hash != super::registry::hash_coordination_registry(registry)
        || state.active_session_count != registry.active_sessions.len()
    {
        return Err(CoordinationError::RegistryContinuityMismatch);
    }
    Ok(())
}
