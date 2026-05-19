use serde::{Deserialize, Serialize};

use super::errors::CoordinationError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationPolicy {
    pub exchanges_allowed: bool,
}

pub fn verify_coordination_policy(
    policy: &CoordinationPolicy,
    exchange_attempted: bool,
) -> Result<(), CoordinationError> {
    if exchange_attempted && !policy.exchanges_allowed {
        return Err(CoordinationError::PolicyViolation);
    }
    Ok(())
}
