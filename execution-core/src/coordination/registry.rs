use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{canonical::encoding::canonical_encode, operator::continuity::Hash256};

use super::{errors::CoordinationError, session::CoordinationSession};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationRegistry {
    pub active_sessions: BTreeMap<Hash256, CoordinationSession>,
}

pub fn hash_coordination_registry(registry: &CoordinationRegistry) -> Hash256 {
    Sha256::digest(&canonical_encode(registry).expect("coordination registry encode")).into()
}

pub fn update_coordination_registry(
    registry: &CoordinationRegistry,
    session: CoordinationSession,
) -> Result<CoordinationRegistry, CoordinationError> {
    if registry.active_sessions.contains_key(&session.session_id) {
        return Err(CoordinationError::DuplicateSession);
    }
    let mut next = registry.clone();
    next.active_sessions.insert(session.session_id, session);
    Ok(next)
}
