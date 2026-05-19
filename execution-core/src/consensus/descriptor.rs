use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{canonical::encoding::canonical_encode, operator::continuity::Hash256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusDescriptor {
    pub consensus_id: Hash256,
    pub epoch_hash: Hash256,
    pub quorum_hash: Hash256,
}

pub fn hash_consensus_descriptor(descriptor: &ConsensusDescriptor) -> Hash256 {
    Sha256::digest(&canonical_encode(descriptor).expect("consensus descriptor encode")).into()
}
