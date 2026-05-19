use serde::{Deserialize, Serialize};

use super::{errors::LeaseError, lease::ExecutionLease};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseExpirationReport {
    pub expired: bool,
    pub current_tick: u64,
}

pub fn verify_lease_expiration(
    lease: &ExecutionLease,
    current_tick: u64,
) -> Result<LeaseExpirationReport, LeaseError> {
    if current_tick < lease.lease_start_tick {
        return Err(LeaseError::LeaseNotYetActive);
    }
    if current_tick > lease.lease_end_tick {
        return Err(LeaseError::LeaseExpired);
    }
    Ok(LeaseExpirationReport {
        expired: false,
        current_tick,
    })
}
