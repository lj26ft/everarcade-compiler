use crate::{
    simulation::node::SimulatedNode,
    simulation::sync_session::run_sync_session,
    sync::{SyncRequest, SyncResponse},
};

pub fn simulate_convergence(
    local: &SimulatedNode,
    request: SyncRequest,
    response: SyncResponse,
) -> bool {
    run_sync_session(local, request, response).converged
}
