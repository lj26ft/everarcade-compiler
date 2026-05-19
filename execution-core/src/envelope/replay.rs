use super::errors::EnvelopeError;
use crate::operator::continuity::Hash256;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayProtection {
    pub known_message_ids: BTreeSet<Hash256>,
}
pub fn verify_replay_protection(
    replay: &ReplayProtection,
    message_id: &Hash256,
) -> Result<(), EnvelopeError> {
    if replay.known_message_ids.contains(message_id) {
        return Err(EnvelopeError::ReplayDetected);
    }
    Ok(())
}
