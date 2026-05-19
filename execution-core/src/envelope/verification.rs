use super::{
    errors::EnvelopeError, message::verify_signed_message, policy::verify_envelope_policy,
    quarantine::verify_envelope_quarantine, registry::EnvelopeRegistry,
    replay::verify_replay_protection, signature::verify_continuity_signature,
    state::verify_envelope_state, ContinuitySignature, EnvelopePolicy, EnvelopeQuarantine,
    EnvelopeState, ReplayProtection, SignedContinuityMessage,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvelopeVerificationReport {
    pub valid: bool,
    pub replay_detected: bool,
}
pub fn verify_envelope(
    message: &SignedContinuityMessage,
    signature: &ContinuitySignature,
    registry: &EnvelopeRegistry,
    replay: &ReplayProtection,
    quarantine: &EnvelopeQuarantine,
    policy: &EnvelopePolicy,
    state: &EnvelopeState,
) -> Result<EnvelopeVerificationReport, EnvelopeError> {
    verify_signed_message(message)?;
    verify_continuity_signature(signature, message)?;
    verify_replay_protection(replay, &message.message_id)?;
    verify_envelope_quarantine(quarantine)?;
    verify_envelope_policy(policy, replay)?;
    verify_envelope_state(state, registry)?;
    Ok(EnvelopeVerificationReport {
        valid: true,
        replay_detected: false,
    })
}
