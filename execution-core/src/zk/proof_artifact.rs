use super::Hash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZkProofArtifact {
    pub proof_root: Hash,
    pub statement_root: Hash,
    pub verification_key_root: Hash,
    pub public_inputs_root: Hash,
}
