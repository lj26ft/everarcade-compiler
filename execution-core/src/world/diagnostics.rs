use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentWorldDiagnostics {
    pub world_id: String,
    pub civilization_epoch: u64,
    pub tick_index: u64,
    pub entity_count: usize,
    pub inventory_mutations: usize,
    pub economy_mutations: usize,
    pub checkpoint_count: usize,
    pub archive_size_bytes: usize,
    pub restoration_possible: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeMetrics {
    pub window_hash: String,
    pub progression_cursor: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializationMetrics {
    pub snapshot_roots: Vec<String>,
    pub lane_roots: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayMetrics {
    pub replay_anchors: Vec<String>,
    pub replay_equivalence: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorationMetrics {
    pub restoration_equivalence: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneMetrics {
    pub lane_count: u64,
    pub lane_roots: Vec<String>,
}
