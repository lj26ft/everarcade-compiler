use serde::{Deserialize, Serialize};

use super::errors::CoordinationError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationQuarantine {
    pub quarantined_exchange: bool,
}

pub fn verify_coordination_quarantine(
    quarantine: &CoordinationQuarantine,
    exchange_is_valid: bool,
) -> Result<(), CoordinationError> {
    if !exchange_is_valid && !quarantine.quarantined_exchange {
        return Err(CoordinationError::QuarantineViolation);
    }
    Ok(())
}
