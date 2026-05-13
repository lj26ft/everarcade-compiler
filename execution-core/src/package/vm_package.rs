use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VmPackageManifest {
    pub package_id: Hash,
    pub protocol_version: u64,
    pub civilization_root: Hash,
    pub replay_root: Hash,
    pub checkpoint_root: Hash,
}
