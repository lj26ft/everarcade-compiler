use super::errors::EnvelopeError;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvelopeQuarantine {
    pub quarantined_message: bool,
}
pub fn verify_envelope_quarantine(quarantine: &EnvelopeQuarantine) -> Result<(), EnvelopeError> {
    if quarantine.quarantined_message {
        return Err(EnvelopeError::QuarantineViolation);
    }
    Ok(())
}
