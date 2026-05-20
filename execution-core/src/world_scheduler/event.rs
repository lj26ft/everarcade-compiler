use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorldEventKind {
    World,
    Entity,
    Migration,
    Ownership,
    Scheduler,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldEvent {
    pub sequence: u64,
    pub kind: WorldEventKind,
    pub subject_id: String,
    pub payload: Vec<u8>,
}

pub fn propagate_world_event(events: &mut Vec<WorldEvent>, event: WorldEvent) {
    events.push(event);
    events.sort_by_key(|e| e.sequence);
}

pub fn verify_event_continuity(events: &[WorldEvent]) -> bool {
    events.windows(2).all(|w| w[0].sequence < w[1].sequence)
}

pub fn replay_world_events(events: &[WorldEvent]) -> Vec<WorldEvent> {
    let mut out = events.to_vec();
    out.sort_by_key(|e| e.sequence);
    out
}
