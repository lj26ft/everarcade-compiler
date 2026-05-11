use crate::hashing;

use super::{execution_trace::ExecutionTrace, proof::ExecutionProof};

pub fn deterministic_prove(trace: &ExecutionTrace, receipt_hash: &str, proof_system: &str) -> ExecutionProof {
    let proof_seed = format!("{}:{}:{}:{}", proof_system, trace.epoch_id, trace.execution_root, receipt_hash);
    let proof_bytes = hashing::hash_bytes(proof_seed.as_bytes()).into_bytes();

    ExecutionProof {
        proof_system: proof_system.to_string(),
        execution_root: trace.execution_root.clone(),
        receipt_hash: receipt_hash.to_string(),
        snapshot_hash: trace.snapshot_hash.clone(),
        epoch_id: trace.epoch_id,
        proof_bytes,
    }
}
