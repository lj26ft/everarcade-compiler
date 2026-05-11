use serde::{Deserialize, Serialize};

use super::proof::ExecutionProof;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AggregateProof {
    pub proof_system: String,
    pub epoch_id: u64,
    pub proof_hashes: Vec<String>,
    pub aggregate_hash: String,
}

pub fn aggregate_proofs(proofs: &[ExecutionProof]) -> AggregateProof {
    let proof_system = proofs.first().map(|p| p.proof_system.clone()).unwrap_or_else(|| "none".into());
    let epoch_id = proofs.first().map(|p| p.epoch_id).unwrap_or_default();
    let proof_hashes: Vec<String> = proofs.iter().map(|p| hex::encode(&p.proof_bytes)).collect();
    let aggregate_hash = crate::hashing::hash_bytes(proof_hashes.join("|").as_bytes());

    AggregateProof { proof_system, epoch_id, proof_hashes, aggregate_hash }
}
