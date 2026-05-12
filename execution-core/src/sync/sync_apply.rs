use crate::sync::{sync_reduce::reduce_sync_exchange, sync_result::SyncResult, SyncRequest, SyncResponse, SyncStatus};

pub fn apply_sync_exchange(local: SyncStatus, request: SyncRequest, response: SyncResponse) -> SyncResult {
    reduce_sync_exchange(local, request, response)
}
