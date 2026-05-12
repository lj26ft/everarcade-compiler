use serde::{Deserialize, Serialize};

use super::aggregated_proof::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofChain {
    pub head: Hash,
    pub history: Vec<Hash>,
}
