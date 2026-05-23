use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldPartition {
    pub partition_id: String,
    pub routing_boundary: String,
    pub sync_window: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TopologyManifest {
    pub world_id: String,
    pub partitions: Vec<WorldPartition>,
    pub prior_topology_root: String,
}

impl TopologyManifest {
    pub fn shard_lineage_hash(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("topology serialization must succeed"))
    }
}
