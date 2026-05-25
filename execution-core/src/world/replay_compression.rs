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
