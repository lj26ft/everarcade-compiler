use super::epoch_resource_summary::EpochResourceSummary;

pub fn total_units(summary: EpochResourceSummary) -> u64 {
    summary.total_execution_units
        .saturating_add(summary.total_replay_units)
        .saturating_add(summary.total_storage_units)
        .saturating_add(summary.total_proof_units)
}
