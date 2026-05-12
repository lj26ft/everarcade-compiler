use crate::{simulation::node::SimulatedNode, sync::{sync_apply::apply_sync_exchange, SyncRequest, SyncResponse, SyncResult}};

pub fn run_sync_session(local: &SimulatedNode, request: SyncRequest, response: SyncResponse) -> SyncResult {
    apply_sync_exchange(local.sync_status.clone(), request, response)
}
