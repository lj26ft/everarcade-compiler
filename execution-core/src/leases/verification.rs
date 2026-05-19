use serde::{Deserialize, Serialize};

use super::{
    errors::LeaseError,
    expiration::verify_lease_expiration,
    lease::ExecutionLease,
    registry::LeaseRegistry,
    window::{verify_lease_window, LeaseWindow},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseVerificationReport {
    pub valid: bool,
    pub expired: bool,
    pub overlapping: bool,
}

pub fn verify_execution_lease(
    lease: &ExecutionLease,
    registry: &LeaseRegistry,
    current_tick: u64,
) -> Result<LeaseVerificationReport, LeaseError> {
    if lease.authority != registry.current_lease.authority {
        return Err(LeaseError::AuthorityMismatch);
    }
    let current = LeaseWindow {
        start_tick: lease.lease_start_tick,
        end_tick: lease.lease_end_tick,
    };
    verify_lease_window(&current, None)?;
    let previous = LeaseWindow {
        start_tick: registry.current_lease.lease_start_tick,
        end_tick: registry.current_lease.lease_end_tick,
    };
    let overlapping =
        lease.epoch != registry.current_lease.epoch && lease.lease_start_tick <= previous.end_tick;
    if overlapping {
        return Err(LeaseError::OverlappingWindow);
    }
    verify_lease_expiration(lease, current_tick)?;
    Ok(LeaseVerificationReport {
        valid: true,
        expired: false,
        overlapping: false,
    })
}
