use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayCompressionWindow {
    pub start_epoch: u64,
    pub end_epoch: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySnapshot {
    pub checkpoint_root: String,
    pub epoch_hashes: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayAnchor {
    pub anchor_epoch: u64,
    pub anchor_hash: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompressedEpochRange {
    pub window: ReplayCompressionWindow,
    pub snapshot: ReplaySnapshot,
    pub anchor: ReplayAnchor,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayWitness {
    pub range_hash: String,
    pub anchor_hash: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionWitness {
    pub execution_id: String,
    pub receipt_hash: String,
    pub mutation_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncrementalReplayWindow {
    pub start_tick: u64,
    pub end_tick: u64,
    pub delta_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayCursor {
    pub next_window_start: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayDelta {
    pub window: IncrementalReplayWindow,
    pub mutation_count: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayMergeBoundary {
    pub merged_until_tick: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayMaterializationCursor {
    pub materialized_until_tick: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StreamingWitnessBundle {
    pub chunks: Vec<WitnessChunk>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessChunk {
    pub chunk_id: u64,
    pub witness_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessSegment {
    pub segment_id: u64,
    pub chunk_ids: Vec<u64>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessCursor {
    pub next_chunk_id: u64,
}
