use crate::{checkpoint::checkpoint_snapshot::CheckpointSnapshot, replay::replay_proof::ReplayProof, receipt_runtime::execution_receipt::ExecutionReceipt};

use super::proof_exchange::{ReceiptProof, StateProof};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyncResponse {
    pub checkpoint: Option<CheckpointSnapshot>,
    pub receipts: Vec<ExecutionReceipt>,
    pub state_proofs: Vec<StateProof>,
    pub receipt_proofs: Vec<ReceiptProof>,
    pub replay_proof: Option<ReplayProof>,
}
