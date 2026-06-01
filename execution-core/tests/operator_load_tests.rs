use control_plane::*;

fn run_load(runtime_count: usize) {
    let mut control = OperatorControlPlane::arena_vanguard();
    for i in 0..runtime_count {
        let result = control
            .orchestrator
            .deploy_runtime(
                format!("arena-{i}"),
                format!("hash-{i}"),
                "1.0",
                vec!["rig".into()],
            )
            .unwrap();
        let mut health = control
            .orchestrator
            .supervisor
            .health(&result.runtime.runtime_id)
            .unwrap();
        assert_eq!(health.evaluate(), HealthState::Healthy);
        control
            .orchestrator
            .supervisor
            .recover_runtime(&mut control.orchestrator.leases, &result.runtime.runtime_id)
            .unwrap();
    }
    control.emit_lease_exhaustion();
    assert_eq!(
        control
            .orchestrator
            .metrics
            .snapshot
            .deployment
            .deployment_count,
        runtime_count as u64
    );
    assert!(!control.alerts.alerts.is_empty());
}

#[test]
fn test_1_runtime_load() {
    run_load(1);
}
#[test]
fn test_5_runtimes_load() {
    run_load(5);
}
#[test]
fn test_10_runtimes_load() {
    run_load(10);
}
#[test]
fn test_25_runtimes_load() {
    run_load(25);
}
#[test]
fn test_50_runtimes_load() {
    run_load(50);
}
