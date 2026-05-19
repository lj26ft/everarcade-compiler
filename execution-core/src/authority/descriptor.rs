use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    canonical::encoding::canonical_encode, federation::node::FederationNodeId,
    operator::continuity::Hash256,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityDescriptor {
    pub authority_node: FederationNodeId,
    pub epoch: u64,
    pub package_root: Hash256,
    pub checkpoint_root: Hash256,
}

pub fn hash_authority_descriptor(descriptor: &AuthorityDescriptor) -> Hash256 {
    Sha256::digest(&canonical_encode(descriptor).expect("authority descriptor encode")).into()
}
