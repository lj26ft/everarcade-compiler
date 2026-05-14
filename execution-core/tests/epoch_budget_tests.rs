use execution_core::epoch::{
    epoch_governance::total_units, epoch_resource_summary::EpochResourceSummary,
};

#[test]
fn stable_epoch_accounting() {
    let summary = EpochResourceSummary {
        total_execution_units: 10,
        total_replay_units: 2,
        total_storage_units: 3,
        total_proof_units: 4,
    };
    assert_eq!(total_units(summary), 19);
}
