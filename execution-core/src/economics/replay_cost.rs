use super::{CostSchedule, ResourceUsage};

pub fn replay_cost(events: u64, schedule: CostSchedule) -> ResourceUsage {
    ResourceUsage {
        replay_units: events.saturating_mul(schedule.replay_unit_per_event),
        ..ResourceUsage::default()
    }
}
