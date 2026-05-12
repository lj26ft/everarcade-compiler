use super::{CostSchedule, ResourceUsage};

pub fn proof_cost(receipts: u64, schedule: CostSchedule) -> ResourceUsage {
    ResourceUsage {
        proof_units: receipts.saturating_mul(schedule.proof_unit_per_receipt),
        ..ResourceUsage::default()
    }
}
