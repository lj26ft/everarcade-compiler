use crate::{simulation::sync_session::run_sync_session, sync::{SyncRequest, SyncResponse}, simulation::node::SimulatedNode};

pub fn simulate_convergence(local: &SimulatedNode, request: SyncRequest, response: SyncResponse) -> bool {
    run_sync_session(local, request, response).converged
}
