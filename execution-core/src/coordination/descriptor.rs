use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{canonical::encoding::canonical_encode, operator::continuity::Hash256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationDescriptor {
    pub coordination_id: Hash256,
    pub session_hash: Hash256,
    pub registry_hash: Hash256,
}

pub fn hash_coordination_descriptor(descriptor: &CoordinationDescriptor) -> Hash256 {
    Sha256::digest(&canonical_encode(descriptor).expect("coordination descriptor encode")).into()
}
