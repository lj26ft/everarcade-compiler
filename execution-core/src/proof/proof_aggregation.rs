use super::{
    aggregated_proof::{AggregatedProof, Hash},
    proof_commitment::proof_commitment,
};

pub fn aggregate_roots(included_roots: Vec<Hash>) -> AggregatedProof {
    let aggregation_root = proof_commitment(included_roots.clone());
    AggregatedProof {
        proof_root: aggregation_root,
        included_roots,
        aggregation_root,
    }
}
