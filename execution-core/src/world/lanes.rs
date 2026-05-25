use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneExecutionQueue {
    pub lane_id: u64,
    pub partitions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneCommitPhase {
    pub lane_id: u64,
    pub committed_partitions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneMergePhase {
    pub merged_lane_ids: Vec<u64>,
    pub merge_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneCheckpointBoundary {
    pub lane_id: u64,
    pub checkpoint_index: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLaneScheduler {
    pub lanes: Vec<LaneExecutionQueue>,
}

impl ExecutionLaneScheduler {
    pub fn from_partitions(partitions: &[String], lane_count: u64) -> Self {
        let mut lanes = (0..lane_count)
            .map(|lane_id| LaneExecutionQueue {
                lane_id,
                partitions: Vec::new(),
            })
            .collect::<Vec<_>>();
        let mut sorted = partitions.to_vec();
        sorted.sort();
        for (idx, p) in sorted.into_iter().enumerate() {
            lanes[idx % lane_count as usize].partitions.push(p);
        }
        Self { lanes }
    }

    pub fn deterministic_order(&self) -> Vec<String> {
        let mut out = Vec::new();
        for lane in &self.lanes {
            for p in &lane.partitions {
                out.push(format!("{}:{}", lane.lane_id, p));
            }
        }
        out
    }

    pub fn merge_phase(&self) -> Result<LaneMergePhase, String> {
        let lane_ids = self.lanes.iter().map(|l| l.lane_id).collect::<Vec<_>>();
        let root =
            hash_bytes(&canonical_encode(&self.deterministic_order()).map_err(|e| e.to_string())?);
        Ok(LaneMergePhase {
            merged_lane_ids: lane_ids,
            merge_root: root,
        })
    }
}
