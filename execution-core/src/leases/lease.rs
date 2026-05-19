use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    canonical::encoding::canonical_encode, federation::node::FederationNodeId,
    operator::continuity::Hash256,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLease {
    pub authority: FederationNodeId,
    pub epoch: u64,
    pub lease_start_tick: u64,
    pub lease_end_tick: u64,
    pub checkpoint_root: Hash256,
}

pub fn hash_execution_lease(lease: &ExecutionLease) -> Hash256 {
    Sha256::digest(&canonical_encode(lease).expect("execution lease encode")).into()
}
