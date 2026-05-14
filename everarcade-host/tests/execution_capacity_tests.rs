use everarcade_host::evernode::{
    capacity_manifest::ExecutionCapacityManifest, execution_discovery::discover_eligible_operators,
    operator_availability::OperatorAvailability,
};

#[test]
fn only_available_operators_discovered() {
    let manifests = vec![ExecutionCapacityManifest {
        operator_id: [1; 32],
        supported_package_root: [7; 32],
        max_execution_windows: 3,
        latest_checkpoint_root: [8; 32],
    }];
    let statuses = vec![OperatorAvailability {
        operator_id: [1; 32],
        available_windows: 1,
    }];
    assert_eq!(discover_eligible_operators(&manifests, &statuses).len(), 1);
}
