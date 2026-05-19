use serde::{Deserialize, Serialize};

use super::{errors::LeaseError, lease::ExecutionLease};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeasePolicy {
    pub single_active_lease: bool,
}

pub fn verify_lease_policy(
    policy: &LeasePolicy,
    current: &ExecutionLease,
    candidate: &ExecutionLease,
) -> Result<(), LeaseError> {
    if policy.single_active_lease && candidate.lease_start_tick <= current.lease_end_tick {
        return Err(LeaseError::PolicyViolation);
    }
    Ok(())
}
