use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

use super::metrics::RuntimeMetrics;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeValidationReport {
    pub runtime_hash: String,
    pub metrics: RuntimeMetrics,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochValidationReport {
    pub epoch_count: u64,
    pub checkpoint_lineage_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayValidationReport {
    pub replay_window_count: u64,
    pub replay_equivalence: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityValidationReport {
    pub continuity_root: String,
    pub continuity_equivalence: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneValidationReport {
    pub lane_count: u64,
    pub deterministic_merge_equivalence: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotValidationReport {
    pub snapshot_count: u64,
    pub snapshot_chain_root: String,
}

impl RuntimeValidationReport {
    pub fn from_metrics(metrics: RuntimeMetrics) -> Result<Self, String> {
        let runtime_hash = metrics.runtime_hash()?;
        Ok(Self {
            runtime_hash,
            metrics,
        })
    }

    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}
