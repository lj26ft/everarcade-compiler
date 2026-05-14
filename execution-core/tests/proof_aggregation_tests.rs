use execution_core::proof::{
    proof_aggregation::aggregate_roots, proof_validation::validate_aggregated_proof,
};

#[test]
fn aggregated_proofs_validate_deterministically() {
    let proof = aggregate_roots(vec![[1; 32], [2; 32], [3; 32]]);
    assert!(validate_aggregated_proof(&proof));
}
