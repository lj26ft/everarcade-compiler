use serde::{Deserialize, Serialize};

use crate::operator::continuity::Hash256;

use super::{
    errors::LeaseError,
    lease::{hash_execution_lease, ExecutionLease},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseRenewal {
    pub previous_lease_hash: Hash256,
    pub renewed_lease: ExecutionLease,
}

pub fn verify_lease_renewal(
    previous: &ExecutionLease,
    renewal: &LeaseRenewal,
    handoff: bool,
) -> Result<(), LeaseError> {
    if renewal.previous_lease_hash != hash_execution_lease(previous) {
        return Err(LeaseError::RenewalHashMismatch);
    }
    if !handoff && renewal.renewed_lease.authority != previous.authority {
        return Err(LeaseError::AuthorityMismatch);
    }
    if renewal.renewed_lease.epoch != previous.epoch + 1 {
        return Err(LeaseError::EpochContinuityMismatch);
    }
    if renewal.renewed_lease.lease_start_tick != previous.lease_end_tick + 1
        || renewal.renewed_lease.lease_end_tick < renewal.renewed_lease.lease_start_tick
    {
        return Err(LeaseError::RenewalContinuityMismatch);
    }
    Ok(())
}
