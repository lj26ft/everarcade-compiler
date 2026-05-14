use crate::{
    checkpoint::checkpoint_snapshot::CheckpointSnapshot,
    receipt_runtime::execution_receipt::ExecutionReceipt, replay::replay_proof::ReplayProof,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckpointExchange {
    pub snapshot: CheckpointSnapshot,
    pub continuation_receipts: Vec<ExecutionReceipt>,
    pub replay_proof: ReplayProof,
}
