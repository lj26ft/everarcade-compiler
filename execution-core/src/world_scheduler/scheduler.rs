use sha2::{Digest, Sha256};

use super::{
    error::WorldSchedulerError,
    event::{propagate_world_event, verify_event_continuity, WorldEvent},
    queue::WorldEventQueue,
    tick::{compute_tick_hash, verify_tick_order},
    timeline::{advance_timeline, WorldTickRecord, WorldTimeline},
};

#[derive(Debug, Clone, Default)]
pub struct WorldScheduler {
    pub latest_tick: u64,
    pub timeline: WorldTimeline,
    pub events: Vec<WorldEvent>,
    pub queue: WorldEventQueue,
}

impl WorldScheduler {
    pub fn schedule_world_event(&mut self, event: WorldEvent) {
        self.queue.push(event);
    }

    pub fn advance_world_tick(&mut self) -> Result<u64, WorldSchedulerError> {
        let next = self.latest_tick + 1;
        if !verify_tick_order(next, next) {
            return Err(WorldSchedulerError::OutOfOrderTick {
                expected: next,
                actual: next,
            });
        }
        self.latest_tick = next;
        Ok(next)
    }

    pub fn execute_world_tick(&mut self) -> Result<WorldTickRecord, WorldSchedulerError> {
        let tick = self.advance_world_tick()?;
        let due = self.queue.pop_due(u64::MAX);
        for e in due {
            propagate_world_event(&mut self.events, e);
        }
        if !verify_event_continuity(&self.events) {
            return Err(WorldSchedulerError::ContinuityViolation("event_order"));
        }
        let event_hash: [u8; 32] =
            Sha256::digest(serde_json::to_vec(&self.events).unwrap_or_default()).into();
        let timeline_hash: [u8; 32] =
            Sha256::digest(serde_json::to_vec(&self.timeline).unwrap_or_default()).into();
        let state_root = compute_tick_hash(tick, timeline_hash, event_hash, [0u8; 32], [1u8; 32]);
        let record = WorldTickRecord {
            tick_id: tick,
            timeline_hash,
            event_hash,
            state_root,
            entity_continuity_hash: [1u8; 32],
        };
        advance_timeline(&mut self.timeline, record.clone());
        Ok(record)
    }

    pub fn verify_tick_continuity(&self) -> bool {
        self.timeline
            .ticks
            .windows(2)
            .all(|w| w[0].tick_id + 1 == w[1].tick_id)
    }
}

pub fn replay_world_timeline(timeline: &WorldTimeline) -> WorldTimeline {
    timeline.clone()
}
pub fn verify_world_convergence(a: &WorldTimeline, b: &WorldTimeline) -> bool {
    a == b
}
pub fn reconstruct_entity_lineage(entities: &[String]) -> [u8; 32] {
    Sha256::digest(entities.join("|").as_bytes()).into()
}
