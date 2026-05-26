use crate::{hashing::hash_bytes, wasm::execution::ExecutionStatus};
use serde::{Deserialize, Serialize};

use super::{
    continuity::{ContinuityCursor, ContinuitySegment, ContinuityWindow},
    events::{EventSegment, EventWindow, StreamingEventArchive},
    lanes::ExecutionLaneScheduler,
    replay_compression::{IncrementalReplayWindow, ReplayCursor, StreamingWitnessBundle},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRuntimeTick {
    pub tick: u64,
    pub workload_partitions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRuntimeWindow {
    pub start_tick: u64,
    pub end_tick: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeCheckpointWindow {
    pub checkpoint_index: u64,
    pub window: WorldRuntimeWindow,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeExecutionCursor {
    pub next_tick: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeCommitCursor {
    pub committed_tick: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncrementalWorldRuntime {
    pub execution_cursor: RuntimeExecutionCursor,
    pub commit_cursor: RuntimeCommitCursor,
    pub continuity_cursor: ContinuityCursor,
    pub archive: StreamingEventArchive,
    pub replay_cursor: ReplayCursor,
    pub witnesses: StreamingWitnessBundle,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeGovernancePolicy {
    pub max_mutations_per_execution: u64,
    pub max_mutation_bytes: u64,
    pub max_events_per_execution: u64,
    pub max_event_chunk_size: u64,
    pub max_event_window_size: u64,
    pub max_witness_chunk_size: u64,
    pub max_witness_chain_depth: u64,
    pub max_replay_window_depth: u64,
    pub max_snapshot_chain_depth: u64,
    pub max_partition_merge_inputs: u64,
    pub max_validation_export_size: u64,
}

impl Default for RuntimeGovernancePolicy {
    fn default() -> Self {
        Self {
            max_mutations_per_execution: 1024,
            max_mutation_bytes: 64 * 1024,
            max_events_per_execution: 1024,
            max_event_chunk_size: 16 * 1024,
            max_event_window_size: 1024,
            max_witness_chunk_size: 16 * 1024,
            max_witness_chain_depth: 2048,
            max_replay_window_depth: 2048,
            max_snapshot_chain_depth: 512,
            max_partition_merge_inputs: 128,
            max_validation_export_size: 128 * 1024,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceReceipt {
    pub status: ExecutionStatus,
    pub rejection_root: String,
    pub quarantined: bool,
}

impl GovernanceReceipt {
    fn accepted() -> Self {
        Self {
            status: ExecutionStatus::Success,
            rejection_root: hash_bytes(b"governance:accepted"),
            quarantined: false,
        }
    }

    fn rejected(status: ExecutionStatus, reason: &str) -> Self {
        Self {
            status,
            rejection_root: hash_bytes(reason.as_bytes()),
            quarantined: true,
        }
    }
}

impl Default for IncrementalWorldRuntime {
    fn default() -> Self {
        Self {
            execution_cursor: RuntimeExecutionCursor { next_tick: 0 },
            commit_cursor: RuntimeCommitCursor { committed_tick: 0 },
            continuity_cursor: ContinuityCursor::default(),
            archive: StreamingEventArchive::default(),
            replay_cursor: ReplayCursor {
                next_window_start: 0,
            },
            witnesses: StreamingWitnessBundle::default(),
        }
    }
}

impl IncrementalWorldRuntime {
    pub fn enforce_governance(
        &self,
        policy: &RuntimeGovernancePolicy,
        mutation_count: u64,
        mutation_bytes: u64,
        event_count: u64,
        event_chunk_size: u64,
        witness_chunk_size: u64,
        partition_merge_inputs: u64,
        validation_export_size: u64,
        capability_authorized: bool,
        isolation_ok: bool,
    ) -> GovernanceReceipt {
        if !capability_authorized {
            return GovernanceReceipt::rejected(
                ExecutionStatus::CapabilityViolation,
                "governance:capability_violation",
            );
        }
        if !isolation_ok {
            return GovernanceReceipt::rejected(
                ExecutionStatus::IsolationViolation,
                "governance:isolation_violation",
            );
        }
        if mutation_count > policy.max_mutations_per_execution
            || mutation_bytes > policy.max_mutation_bytes
        {
            return GovernanceReceipt::rejected(
                ExecutionStatus::ResourceLimitExceeded,
                "governance:mutation_overflow",
            );
        }
        if event_count > policy.max_events_per_execution
            || event_chunk_size > policy.max_event_chunk_size
            || event_count > policy.max_event_window_size
        {
            return GovernanceReceipt::rejected(
                ExecutionStatus::EventOverflow,
                "governance:event_overflow",
            );
        }
        if witness_chunk_size > policy.max_witness_chunk_size
            || self.witnesses.chunks.len() as u64 > policy.max_witness_chain_depth
        {
            return GovernanceReceipt::rejected(
                ExecutionStatus::WitnessOverflow,
                "governance:witness_overflow",
            );
        }
        if self.archive.segments.len() as u64 > policy.max_replay_window_depth {
            return GovernanceReceipt::rejected(
                ExecutionStatus::ReplayOverflow,
                "governance:replay_overflow",
            );
        }
        if self.continuity_cursor.segments.len() as u64 > policy.max_snapshot_chain_depth {
            return GovernanceReceipt::rejected(
                ExecutionStatus::SnapshotOverflow,
                "governance:snapshot_overflow",
            );
        }
        if partition_merge_inputs > policy.max_partition_merge_inputs {
            return GovernanceReceipt::rejected(
                ExecutionStatus::ResourceLimitExceeded,
                "governance:partition_merge_overflow",
            );
        }
        if validation_export_size > policy.max_validation_export_size {
            return GovernanceReceipt::rejected(
                ExecutionStatus::ResourceLimitExceeded,
                "governance:validation_export_overflow",
            );
        }
        GovernanceReceipt::accepted()
    }
    pub fn advance(
        &mut self,
        tick: WorldRuntimeTick,
        lane_count: u64,
    ) -> Result<ContinuitySegment, String> {
        let scheduler =
            ExecutionLaneScheduler::from_partitions(&tick.workload_partitions, lane_count.max(1));
        let lane_merge = scheduler.merge_phase()?;
        let event_window = EventWindow {
            start_sequence: tick.tick,
            end_sequence: tick.tick,
            partition_count: tick.workload_partitions.len() as u64,
        };
        let segment =
            EventSegment::from_window(event_window.clone(), lane_merge.merge_root.clone())?;
        self.archive.push_segment(segment.clone());
        self.execution_cursor.next_tick = tick.tick + 1;
        self.commit_cursor.committed_tick = tick.tick;
        self.replay_cursor.next_window_start = tick.tick + 1;

        let continuity_segment = ContinuitySegment {
            tick: tick.tick,
            window: ContinuityWindow {
                start_tick: tick.tick,
                end_tick: tick.tick,
            },
            event_root: segment.segment_root.clone(),
        };
        self.continuity_cursor
            .segments
            .push(continuity_segment.clone());
        self.witnesses
            .chunks
            .push(super::replay_compression::WitnessChunk {
                chunk_id: tick.tick,
                witness_root: segment.segment_root.clone(),
            });
        let _replay_window = IncrementalReplayWindow {
            start_tick: tick.tick,
            end_tick: tick.tick,
            delta_root: segment.segment_root.clone(),
        };
        Ok(continuity_segment)
    }

    pub fn window(&self, size: u64) -> WorldRuntimeWindow {
        let end = self.commit_cursor.committed_tick;
        let start = end.saturating_sub(size.saturating_sub(1));
        WorldRuntimeWindow {
            start_tick: start,
            end_tick: end,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldSessionIdentity { pub world_id: String, pub session_id: String }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeSession { pub identity: WorldSessionIdentity, pub started_tick: u64, pub last_tick: u64 }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeContinuityState { pub continuity_root: String, pub validation_root: String, pub replay_root: String }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentWorldRuntime { pub session: RuntimeSession, pub continuity: RuntimeContinuityState }
