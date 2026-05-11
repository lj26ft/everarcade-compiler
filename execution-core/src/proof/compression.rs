use super::{aggregation::AggregateProof, proof::ExecutionProof};

pub fn compression_ratio(proofs: &[ExecutionProof], aggregate: &AggregateProof) -> f64 {
    let raw: usize = proofs.iter().map(|p| p.proof_bytes.len()).sum();
    if raw == 0 {
        return 1.0;
    }
    aggregate.aggregate_hash.len() as f64 / raw as f64
}
