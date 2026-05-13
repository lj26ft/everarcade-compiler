use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvernodeAnchor {
    pub host_id: Hash,
    pub instance_root: Hash,
    pub anchored_root: Hash,
}
