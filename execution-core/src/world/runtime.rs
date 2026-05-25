use serde::{Deserialize, Serialize};

use super::{
    continuity::{ContinuityCursor, ContinuitySegment, ContinuityWindow},
    events::{EventSegment, EventStreamCursor, EventWindow, StreamingEventArchive},
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
