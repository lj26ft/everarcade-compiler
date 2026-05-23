use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScheduledTick {
    pub tick: u64,
    pub dag_node: String,
    pub sync_window: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchedulingManifest {
    pub federation_id: String,
    pub execution_window_start: u64,
    pub execution_window_end: u64,
    pub ticks: Vec<ScheduledTick>,
}

impl SchedulingManifest {
    pub fn scheduling_hash(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("scheduling serialization must succeed"))
    }
}
