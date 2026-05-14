use super::proof::ExecutionProof;

pub fn verify_proof_integrity(proof: &ExecutionProof) -> bool {
    !proof.proof_system.is_empty() && !proof.proof_bytes.is_empty()
}

pub fn verify_receipt_binding(proof: &ExecutionProof, expected_receipt_hash: &str) -> bool {
    proof.receipt_hash == expected_receipt_hash
}

pub fn verify_execution_root(proof: &ExecutionProof, expected_execution_root: &str) -> bool {
    proof.execution_root == expected_execution_root
}

pub fn verify_epoch(proof: &ExecutionProof, expected_epoch: u64) -> bool {
    proof.epoch_id == expected_epoch
}
