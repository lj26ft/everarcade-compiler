use super::runtime::{ScheduledWork, SimulationSchedulerRuntime};
pub fn schedule_is_deterministic(schedule: &[ScheduledWork]) -> bool {
    schedule
        .windows(2)
        .all(|w| (w[0].priority, &w[0].partition_id) <= (w[1].priority, &w[1].partition_id))
}
pub fn scheduler_equivalent(
    a: &SimulationSchedulerRuntime,
    b: &SimulationSchedulerRuntime,
) -> bool {
    a == b
}
