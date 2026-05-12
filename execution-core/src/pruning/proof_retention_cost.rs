pub fn proof_retention_cost(retained_proofs: u64, per_proof_cost: u64) -> u64 { retained_proofs.saturating_mul(per_proof_cost) }
