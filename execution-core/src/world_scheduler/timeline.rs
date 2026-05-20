use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldTickRecord {
    pub tick_id: u64,
    pub timeline_hash: [u8; 32],
    pub event_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub entity_continuity_hash: [u8; 32],
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldTimeline {
    pub ticks: Vec<WorldTickRecord>,
}

pub fn advance_timeline(timeline: &mut WorldTimeline, record: WorldTickRecord) {
    timeline.ticks.push(record);
    timeline.ticks.sort_by_key(|t| t.tick_id);
}
