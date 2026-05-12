use super::{CostSchedule, ResourceUsage};

pub fn storage_cost(bytes: u64, schedule: CostSchedule) -> ResourceUsage {
    ResourceUsage {
        storage_units: bytes.saturating_mul(schedule.storage_unit_per_byte),
        ..ResourceUsage::default()
    }
}
