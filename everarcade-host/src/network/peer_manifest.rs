use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PeerManifest {
    pub protocol_version: u32,
    pub capability_roots: Vec<[u8; 32]>,
}
