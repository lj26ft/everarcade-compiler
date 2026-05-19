use super::{errors::EnvelopeError, registry::EnvelopeRegistry};
use crate::{canonical::encoding::canonical_encode, operator::continuity::Hash256};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvelopeState {
    pub registry_hash: Hash256,
    pub known_message_count: usize,
}
pub fn hash_envelope_state(state: &EnvelopeState) -> Hash256 {
    Sha256::digest(&canonical_encode(state).expect("envelope state encode")).into()
}
pub fn verify_envelope_state(
    state: &EnvelopeState,
    registry: &EnvelopeRegistry,
) -> Result<(), EnvelopeError> {
    if state.known_message_count != registry.known_messages.len() {
        return Err(EnvelopeError::StateMismatch);
    }
    if state.registry_hash != super::registry::hash_envelope_registry(registry) {
        return Err(EnvelopeError::RegistryContinuityMismatch);
    }
    Ok(())
}
