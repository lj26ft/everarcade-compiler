use crate::hashing;

use super::{execution_trace::ExecutionTrace, proof::ExecutionProof};

pub fn trace_commitment(trace: &ExecutionTrace) -> String {
    hashing::hash_bytes(&trace.canonical_bytes())
}

pub fn snapshot_commitment(snapshot_hash: &str) -> String {
    hashing::hash_bytes(snapshot_hash.as_bytes())
}

pub fn execution_commitment(execution_root: &str, trace_commitment: &str) -> String {
    hashing::hash_bytes(format!("{execution_root}:{trace_commitment}").as_bytes())
}

pub fn receipt_commitment(receipt_hash: &str, proof: &ExecutionProof) -> String {
    hashing::hash_bytes(
        format!(
            "{}:{}",
            receipt_hash,
            hashing::hash_bytes(&proof.proof_bytes)
        )
        .as_bytes(),
    )
}
