use super::Hash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZkProofIntent {
    pub statement_root: Hash,
    pub public_inputs_root: Hash,
}
