use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    canonical::encoding::canonical_encode, federation::node::FederationNodeId,
    operator::continuity::Hash256,
};

use super::errors::CoordinationError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationSession {
    pub session_id: Hash256,
    pub participants: BTreeSet<FederationNodeId>,
}

pub fn hash_coordination_session(session: &CoordinationSession) -> Hash256 {
    Sha256::digest(&canonical_encode(&session.participants).expect("coordination session encode"))
        .into()
}

pub fn verify_coordination_session(session: &CoordinationSession) -> Result<(), CoordinationError> {
    if session.participants.is_empty() {
        return Err(CoordinationError::EmptyParticipants);
    }
    if session.session_id != hash_coordination_session(session) {
        return Err(CoordinationError::SessionIdMismatch);
    }
    Ok(())
}
