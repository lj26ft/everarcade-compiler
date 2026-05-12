use super::{CostSchedule, ResourceUsage};

pub fn execution_cost(steps: u64, schedule: CostSchedule) -> ResourceUsage {
    ResourceUsage {
        execution_units: steps.saturating_mul(schedule.execution_unit_per_step),
        ..ResourceUsage::default()
    }
}
