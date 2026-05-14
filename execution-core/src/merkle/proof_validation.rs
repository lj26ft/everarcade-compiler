use super::{inclusion_proof::verify_inclusion_proof, inclusion_proof::InclusionProof, Hash};

pub fn validate_proof(root: Hash, leaf: Hash, proof: &InclusionProof) -> bool {
    verify_inclusion_proof(root, leaf, proof)
}
