use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PartitionSchedule {
    pub partition_id: String,
    pub tick: u64,
    pub events: Vec<String>,
}

pub fn schedule_partition_execution(
    partition_id: &str,
    tick: u64,
    mut events: Vec<String>,
) -> PartitionSchedule {
    events.sort();
    PartitionSchedule {
        partition_id: partition_id.to_string(),
        tick,
        events,
    }
}

pub fn advance_partition_timeline(current_tick: u64) -> u64 {
    current_tick + 1
}

pub fn verify_partition_schedule(schedule: &PartitionSchedule) -> bool {
    let mut sorted = schedule.events.clone();
    sorted.sort();
    sorted == schedule.events
}
