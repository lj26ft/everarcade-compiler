use serde::{Deserialize, Serialize};

use super::fork::{hash_continuity_fork, ContinuityFork, Hash256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DivergenceProof {
    pub fork_hash: Hash256,
    pub finalized_checkpoint_a: Hash256,
    pub finalized_checkpoint_b: Hash256,
}

pub fn verify_divergence_proof(proof: &DivergenceProof, fork: &ContinuityFork) -> bool {
    proof.fork_hash == hash_continuity_fork(fork)
        && proof.finalized_checkpoint_a == fork.checkpoint_a
        && proof.finalized_checkpoint_b == fork.checkpoint_b
        && proof.finalized_checkpoint_a != proof.finalized_checkpoint_b
}
