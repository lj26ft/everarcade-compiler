use super::{sync_request::SyncRequest, sync_status::SyncStatus};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyncAction { SendReceiptRange, SendCheckpointAndRange, RequestFullReplay, RequestProofOnly, RejectIncompatibleRoots }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyncPlan { pub action: SyncAction, pub from_index: u64, pub to_index: Option<u64> }

pub fn build_sync_plan(request: SyncRequest, local_summary: SyncStatus) -> SyncPlan {
    let action = if request.from_index > local_summary.next_index {
        SyncAction::RejectIncompatibleRoots
    } else if request.local_replay_root == local_summary.replay_root {
        SyncAction::RequestProofOnly
    } else if request.local_state_root == local_summary.state_root {
        SyncAction::SendReceiptRange
    } else if request.local_receipt_root == local_summary.receipt_root {
        SyncAction::RequestFullReplay
    } else {
        SyncAction::SendCheckpointAndRange
    };
    SyncPlan { action, from_index: request.from_index, to_index: request.to_index }
}
