use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncrementalSnapshot {
    pub base_root: String,
    pub deltas: Vec<SnapshotDelta>,
    pub manifest: SnapshotSegmentManifest,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotDelta {
    pub partition_id: String,
    pub mutation_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotSegment {
    pub segment_id: String,
    pub partition_id: String,
    pub segment_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotSegmentManifest {
    pub segment_roots: Vec<String>,
}

impl IncrementalSnapshot {
    pub fn root(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicParallelExecutor;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParallelMergeBarrier {
    pub barrier_tick: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParallelWitnessBoundary {
    pub boundary_chunk: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParallelReplayBoundary {
    pub boundary_tick: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeSnapshot {
    pub world_state_root: String,
    pub entity_state_root: String,
    pub inventory_state_root: String,
    pub event_window_root: String,
    pub replay_window_root: String,
    pub validation_root: String,
    pub continuity_root: String,
    pub checkpoint_lineage_root: String,
}
impl RuntimeSnapshot { pub fn deterministic_hash(&self) -> Result<String,String>{ Ok(hash_bytes(&canonical_encode(self).map_err(|e|e.to_string())?)) } }
