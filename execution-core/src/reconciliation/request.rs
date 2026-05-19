use serde::{Deserialize, Serialize};

use crate::{divergence::fork::Hash256, federation::node::FederationNodeId};

use super::registry::ReconciliationRegistry;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationRequest {
    pub fork_hash: Hash256,
    pub requested_by: FederationNodeId,
}

pub fn verify_reconciliation_request(
    request: &ReconciliationRequest,
    registry: &ReconciliationRegistry,
) -> bool {
    registry
        .quarantined_forks
        .get(&request.fork_hash)
        .map(|descriptor| !descriptor.reconciliation_allowed)
        .unwrap_or(false)
}
