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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreValidationRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeRecoveryWindow {
    pub start_tick: u64,
    pub end_tick: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoredRuntimeState {
    pub world_id: String,
    pub continuity_root: String,
    pub validation_root: RestoreValidationRoot,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRestoreReceipt {
    pub restored: RestoredRuntimeState,
    pub recovery_window: RuntimeRecoveryWindow,
    pub equivalent: bool,
}
