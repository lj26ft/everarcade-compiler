use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VmExecutionInput {
    pub package_manifest_root: Hash,
    pub civilization_root: Hash,
    pub replay_root: Hash,
    pub checkpoint_root: Hash,
    pub payload_root: Hash,
}
