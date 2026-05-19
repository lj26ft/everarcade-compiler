use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::canonical::{encoding::canonical_encode, hashes::Hash};

pub type Hash256 = Hash;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ContinuityFork {
    pub checkpoint_a: Hash256,
    pub checkpoint_b: Hash256,
    pub divergence_tick: u64,
    pub shared_ancestor: Hash256,
}

pub fn hash_continuity_fork(fork: &ContinuityFork) -> Hash256 {
    Sha256::digest(&canonical_encode(fork).expect("continuity fork encode")).into()
}
