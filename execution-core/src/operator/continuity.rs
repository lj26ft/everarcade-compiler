use serde::{Deserialize, Serialize};

use crate::canonical::{encoding::canonical_encode, hashes::Hash};

pub type Hash256 = Hash;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRecoveryDescriptor {
    pub world_id: Hash256,
    pub package_root: Hash256,
    pub latest_checkpoint_root: Hash256,
    pub latest_execution_id: Hash256,
    pub manifest_hash: Hash256,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredRecoveryDescriptor {
    pub descriptor: WorldRecoveryDescriptor,
    pub descriptor_hash: Hash256,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorRecoveryReport {
    pub recovery_ok: bool,
    pub checkpoint_match: bool,
    pub lineage_match: bool,
    pub manifest_match: bool,
    pub replay_match: bool,
    pub recovered_state_root: Hash256,
    pub expected_state_root: Hash256,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorRecoveryMismatch {
    pub field: String,
    pub expected: String,
    pub actual: String,
}

pub fn descriptor_hash(descriptor: &WorldRecoveryDescriptor) -> Hash256 {
    use sha2::{Digest, Sha256};
    Sha256::digest(&canonical_encode(descriptor).expect("descriptor encode")).into()
}
