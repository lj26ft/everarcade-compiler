use crate::merkle::{inclusion_proof::InclusionProof, Hash};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayProof {
    pub receipt_root: Hash,
    pub leaf_hash: Hash,
    pub inclusion_proof: InclusionProof,
}
