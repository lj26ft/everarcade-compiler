use crate::{checkpoint::checkpoint_snapshot::CheckpointSnapshot, replay::replay_proof::ReplayProof, receipt_runtime::execution_receipt::ExecutionReceipt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckpointExchange {
    pub snapshot: CheckpointSnapshot,
    pub continuation_receipts: Vec<ExecutionReceipt>,
    pub replay_proof: ReplayProof,
}
