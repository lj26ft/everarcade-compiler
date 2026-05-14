use crate::replay_sync::sync_result::SyncResult;

pub fn catchup(receipt_count: usize, verified: bool) -> SyncResult {
    SyncResult {
        imported_receipts: receipt_count,
        replay_verified: verified,
    }
}
