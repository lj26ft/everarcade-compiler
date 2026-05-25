use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub execution_count: u64,
    pub partition_count: u64,
    pub aggregated_receipt_root: String,
    pub aggregated_mutation_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochMetrics {
    pub epoch_count: u64,
    pub checkpoint_lineage_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneMetrics {
    pub lane_count: u64,
    pub deterministic_merge_equivalence: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayMetrics {
    pub replay_window_count: u64,
    pub replay_equivalence: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorationMetrics {
    pub restoration_equivalence: bool,
    pub partial_restoration_window_count: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessMetrics {
    pub witness_chunk_count: u64,
    pub aggregated_witness_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventMetrics {
    pub event_chunk_count: u64,
    pub aggregated_event_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotMetrics {
    pub snapshot_count: u64,
    pub snapshot_chain_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityMetrics {
    pub continuity_equivalence: bool,
    pub continuity_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeMetrics {
    pub execution: ExecutionMetrics,
    pub epoch: EpochMetrics,
    pub lane: LaneMetrics,
    pub replay: ReplayMetrics,
    pub restoration: RestorationMetrics,
    pub witness: WitnessMetrics,
    pub event: EventMetrics,
    pub snapshot: SnapshotMetrics,
    pub continuity: ContinuityMetrics,
}

impl RuntimeMetrics {
    pub fn runtime_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}
