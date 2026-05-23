use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TopologyNode {
    pub node_id: String,
    pub node_continuity_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FederationTopologyManifest {
    pub federation_epoch: u64,
    pub nodes: Vec<TopologyNode>,
    pub federation_routing_hash: String,
    pub capability_root: String,
}

impl FederationTopologyManifest {
    pub fn canonicalize(&mut self) {
        self.nodes.sort();
    }

    pub fn canonical_hash(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("topology serialize failed"))
    }
}
