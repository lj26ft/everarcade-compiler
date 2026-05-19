use super::{errors::EnvelopeError, message::SignedContinuityMessage};
use crate::{canonical::encoding::canonical_encode, operator::continuity::Hash256};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvelopeRegistry {
    pub known_messages: BTreeMap<Hash256, SignedContinuityMessage>,
}
pub fn hash_envelope_registry(registry: &EnvelopeRegistry) -> Hash256 {
    Sha256::digest(&canonical_encode(registry).expect("envelope registry encode")).into()
}
pub fn update_envelope_registry(
    registry: &EnvelopeRegistry,
    message: SignedContinuityMessage,
) -> Result<EnvelopeRegistry, EnvelopeError> {
    if registry.known_messages.contains_key(&message.message_id) {
        return Err(EnvelopeError::DuplicateMessage);
    }
    let mut next = registry.clone();
    next.known_messages.insert(message.message_id, message);
    Ok(next)
}
