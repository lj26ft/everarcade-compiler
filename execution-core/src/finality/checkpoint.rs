use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    canonical::encoding::canonical_encode, federation::node::FederationNodeId,
    operator::continuity::Hash256,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedCheckpoint {
    pub checkpoint_root: Hash256,
    pub execution_id: Hash256,
    pub finalized_tick: u64,
    pub acknowledged_observers: BTreeSet<FederationNodeId>,
}

pub fn hash_finalized_checkpoint(checkpoint: &FinalizedCheckpoint) -> Hash256 {
    use sha2::{Digest, Sha256};
    Sha256::digest(&canonical_encode(checkpoint).expect("finalized checkpoint encode")).into()
}
