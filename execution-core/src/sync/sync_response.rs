use crate::{
    checkpoint::checkpoint_snapshot::CheckpointSnapshot,
    receipt_runtime::execution_receipt::ExecutionReceipt, replay::replay_proof::ReplayProof,
};

use super::proof_exchange::{ReceiptProof, StateProof};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyncResponse {
    pub checkpoint: Option<CheckpointSnapshot>,
    pub receipts: Vec<ExecutionReceipt>,
    pub state_proofs: Vec<StateProof>,
    pub receipt_proofs: Vec<ReceiptProof>,
    pub replay_proof: Option<ReplayProof>,
}
