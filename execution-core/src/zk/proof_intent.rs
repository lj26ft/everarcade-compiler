use serde::{Deserialize, Serialize};
use super::Hash;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZkProofIntent {
    pub statement_root: Hash,
    pub public_inputs_root: Hash,
}
