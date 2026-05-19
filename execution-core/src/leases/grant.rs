use serde::{Deserialize, Serialize};

use crate::federation::node::FederationNodeId;

use super::{
    errors::LeaseError,
    lease::ExecutionLease,
    registry::LeaseRegistry,
    window::{verify_lease_window, LeaseWindow},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseGrant {
    pub authority: FederationNodeId,
    pub lease: ExecutionLease,
}

pub fn verify_lease_grant(grant: &LeaseGrant, registry: &LeaseRegistry) -> Result<(), LeaseError> {
    if grant.authority != registry.current_lease.authority
        || grant.lease.authority != grant.authority
    {
        return Err(LeaseError::AuthorityMismatch);
    }
    if grant.lease.epoch != registry.current_lease.epoch + 1 {
        return Err(LeaseError::EpochContinuityMismatch);
    }
    let prev = LeaseWindow {
        start_tick: registry.current_lease.lease_start_tick,
        end_tick: registry.current_lease.lease_end_tick,
    };
    let cur = LeaseWindow {
        start_tick: grant.lease.lease_start_tick,
        end_tick: grant.lease.lease_end_tick,
    };
    verify_lease_window(&cur, Some(&prev))
}
