use serde::{Deserialize, Serialize};

use super::{
    errors::LeaseError,
    lease::ExecutionLease,
    policy::{verify_lease_policy, LeasePolicy},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseRegistry {
    pub current_lease: ExecutionLease,
}

pub fn update_lease_registry(
    registry: &LeaseRegistry,
    next: &ExecutionLease,
) -> Result<LeaseRegistry, LeaseError> {
    if next.epoch != registry.current_lease.epoch + 1 {
        return Err(LeaseError::EpochContinuityMismatch);
    }
    verify_lease_policy(
        &LeasePolicy {
            single_active_lease: true,
        },
        &registry.current_lease,
        next,
    )?;
    Ok(LeaseRegistry {
        current_lease: next.clone(),
    })
}
