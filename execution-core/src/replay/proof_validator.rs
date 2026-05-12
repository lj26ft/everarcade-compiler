use crate::merkle::inclusion_proof::verify_inclusion_proof;

use super::replay_proof::ReplayProof;

pub fn validate_replay_proof(proof: &ReplayProof) -> bool {
    verify_inclusion_proof(proof.receipt_root, proof.leaf_hash, &proof.inclusion_proof)
}
