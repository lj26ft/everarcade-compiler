use super::runtime::{ScheduledWork, SimulationSchedulerRuntime};
pub fn restore_scheduler(schedule: &[ScheduledWork]) -> SimulationSchedulerRuntime {
    SimulationSchedulerRuntime {
        tick: schedule.iter().map(|s| s.tick + 1).max().unwrap_or(0),
        schedule: schedule.to_vec(),
    }
}
