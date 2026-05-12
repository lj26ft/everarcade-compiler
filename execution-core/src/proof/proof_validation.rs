use super::{aggregated_proof::AggregatedProof, proof_commitment::proof_commitment};

pub fn validate_aggregated_proof(proof: &AggregatedProof) -> bool {
    let derived = proof_commitment(proof.included_roots.clone());
    derived == proof.aggregation_root && proof.proof_root == proof.aggregation_root
}
