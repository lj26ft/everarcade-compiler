use super::{errors::EnvelopeError, replay::ReplayProtection};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvelopePolicy {
    pub replay_protection_required: bool,
}
pub fn verify_envelope_policy(
    policy: &EnvelopePolicy,
    replay: &ReplayProtection,
) -> Result<(), EnvelopeError> {
    if policy.replay_protection_required && replay.known_message_ids.is_empty() {
        return Err(EnvelopeError::PolicyViolation);
    }
    Ok(())
}
