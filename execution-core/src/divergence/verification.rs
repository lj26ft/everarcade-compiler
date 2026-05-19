use serde::{Deserialize, Serialize};

use super::{
    detection::detect_divergence,
    fork::ContinuityFork,
    proof::{verify_divergence_proof, DivergenceProof},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DivergenceVerificationReport {
    pub valid: bool,
    pub conflicting_finality: bool,
}

pub fn verify_divergence(
    proof: &DivergenceProof,
    fork: &ContinuityFork,
    continuity_root_a: [u8; 32],
    continuity_root_b: [u8; 32],
) -> DivergenceVerificationReport {
    let proof_valid = verify_divergence_proof(proof, fork);
    let detection = detect_divergence(
        fork.checkpoint_a,
        fork.checkpoint_b,
        continuity_root_a,
        continuity_root_b,
        Some(fork.shared_ancestor),
    );
    DivergenceVerificationReport {
        valid: proof_valid && detection.divergence_detected,
        conflicting_finality: fork.checkpoint_a != fork.checkpoint_b,
    }
}
