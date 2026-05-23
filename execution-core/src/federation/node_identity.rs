use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeIdentityContinuity {
    pub node_id: String,
    pub previous_identity_hash: String,
    pub checkpoint_hash: String,
    pub identity_hash: String,
}

impl NodeIdentityContinuity {
    pub fn new(node_id: String, previous_identity_hash: String, checkpoint_hash: String) -> Self {
        let identity_hash =
            hash_bytes(format!("{node_id}|{previous_identity_hash}|{checkpoint_hash}").as_bytes());
        Self {
            node_id,
            previous_identity_hash,
            checkpoint_hash,
            identity_hash,
        }
    }

    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}
