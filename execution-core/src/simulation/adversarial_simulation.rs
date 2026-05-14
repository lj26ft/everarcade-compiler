use crate::{
    simulation::node::SimulatedNode,
    sync::{sync_apply::apply_sync_exchange, SyncFailure, SyncRequest, SyncResponse},
};

pub fn simulate_adversarial_rejection(
    local: &SimulatedNode,
    request: SyncRequest,
    response: SyncResponse,
) -> bool {
    matches!(
        apply_sync_exchange(local.sync_status.clone(), request, response).failure,
        Some(
            SyncFailure::InvalidProofExchange
                | SyncFailure::InvalidReceiptRange
                | SyncFailure::ReplayRootMismatch
        )
    )
}
