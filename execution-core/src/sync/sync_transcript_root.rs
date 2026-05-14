use crate::merkle::{leaf_hash::leaf_hash, Hash};

pub fn compute_sync_transcript_root(
    request_hash: Hash,
    response_hash: Hash,
    convergence_result_hash: Hash,
) -> Hash {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&request_hash);
    bytes.extend_from_slice(&response_hash);
    bytes.extend_from_slice(&convergence_result_hash);
    leaf_hash(&bytes)
}
