use crate::{checkpoint::checkpoint_snapshot::CheckpointSnapshot, merkle::Hash, receipt_runtime::execution_receipt::ExecutionReceipt, sync::SyncStatus};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimulatedNode {
    pub node_id: Hash,
    pub sync_status: SyncStatus,
    pub checkpoint: Option<CheckpointSnapshot>,
    pub receipts: Vec<ExecutionReceipt>,
}
