use crate::{checkpoint::{checkpoint_restore::restore_checkpoint, checkpoint_snapshot::CheckpointSnapshot, checkpoint_validation::validate_checkpoint}, merkle::Hash, receipt_runtime::execution_receipt::ExecutionReceipt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DivergenceReason { InvalidCheckpoint, RestoreFailed, ReplayRootMismatch }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConvergenceResult { pub converged: bool, pub final_state_root: Hash, pub final_replay_root: Hash, pub final_receipt_root: Hash, pub divergence: Option<DivergenceReason> }

pub fn validate_convergence(_genesis: crate::State, checkpoint: Option<CheckpointSnapshot>, receipts: &[ExecutionReceipt], expected_replay_root: Hash) -> ConvergenceResult {
    if let Some(cp) = checkpoint {
        if !validate_checkpoint(&cp) { return ConvergenceResult { converged: false, final_state_root: cp.state_root, final_replay_root: cp.replay_root, final_receipt_root: cp.receipt_root, divergence: Some(DivergenceReason::InvalidCheckpoint)}; }
        if restore_checkpoint(&cp).is_err() { return ConvergenceResult { converged: false, final_state_root: cp.state_root, final_replay_root: cp.replay_root, final_receipt_root: cp.receipt_root, divergence: Some(DivergenceReason::RestoreFailed)}; }
    }
    let final_receipt_root = crate::merkle::receipt_merkle::receipt_root(receipts);
    let final_replay_root = receipts.last().map(|r| crate::merkle::leaf_hash::leaf_hash(r.replay_root.as_bytes())).unwrap_or([0u8;32]);
    let final_state_root = receipts.last().map(|r| crate::merkle::leaf_hash::leaf_hash(r.state_root.as_bytes())).unwrap_or([0u8;32]);
    let converged = final_replay_root == expected_replay_root;
    ConvergenceResult { converged, final_state_root, final_replay_root, final_receipt_root, divergence: if converged { None } else { Some(DivergenceReason::ReplayRootMismatch) } }
}
