use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AggregatedProof {
    pub proof_root: Hash,
    pub included_roots: Vec<Hash>,
    pub aggregation_root: Hash,
}
