use crate::world::WorldCheckpoint;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorationManifest {
    pub world_id: String,
    pub checkpoint: WorldCheckpoint,
    pub cold_restore: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartialWorldRestoration {
    pub world_id: String,
    pub partitions: Vec<PartitionRestoration>,
    pub segments: Vec<SegmentRestoration>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartitionRestoration {
    pub partition_id: String,
    pub replay_start: u64,
    pub replay_end: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SegmentRestoration {
    pub snapshot_segment_id: String,
    pub event_segment_start: u64,
    pub event_segment_end: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncrementalRestorationReceipt {
    pub world_id: String,
    pub restored_partitions: usize,
    pub restored_segments: usize,
}
