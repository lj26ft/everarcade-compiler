use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{canonical::encoding::canonical_encode, divergence::fork::Hash256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ReconciliationDescriptor {
    pub fork_hash: Hash256,
    pub checkpoint_a: Hash256,
    pub checkpoint_b: Hash256,
    pub reconciliation_allowed: bool,
}

pub fn hash_reconciliation_descriptor(descriptor: &ReconciliationDescriptor) -> Hash256 {
    Sha256::digest(&canonical_encode(descriptor).expect("reconciliation descriptor encode")).into()
}
