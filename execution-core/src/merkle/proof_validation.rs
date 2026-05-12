use super::{inclusion_proof::InclusionProof, inclusion_proof::verify_inclusion_proof, Hash};

pub fn validate_proof(root: Hash, leaf: Hash, proof: &InclusionProof) -> bool {
    verify_inclusion_proof(root, leaf, proof)
}
