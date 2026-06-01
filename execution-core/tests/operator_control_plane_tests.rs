use control_plane::*;

fn sample_resources() -> LeaseResources {
    LeaseResources {
        cpu_cores: 4,
        memory_mb: 8192,
        storage_gb: 200,
        bandwidth_mbps: 100,
    }
}

fn sample_manager() -> LeaseManager {
    LeaseManager::arena_vanguard()
}

#[test]
fn test_lease_allocation() {
    let mut leases = sample_manager();
    let lease = leases.allocate_lease("arena", sample_resources()).unwrap();
    assert_eq!(lease.status, LeaseStatus::Allocated);
}

#[test]
fn test_lease_release() {
    let mut leases = sample_manager();
    let lease = leases.allocate_lease("arena", sample_resources()).unwrap();
    leases.release_lease(&lease.id).unwrap();
    assert_eq!(leases.leases[&lease.id].status, LeaseStatus::Stopped);
}

#[test]
fn test_runtime_deployment() {
    let mut orchestrator = DeploymentOrchestrator::new(sample_manager());
    let result = orchestrator
        .deploy_runtime("arena", "hash", "1.0", vec!["rig".into()])
        .unwrap();
    assert_eq!(result.status, DeploymentStatus::Running);
    assert!(result.steps.contains(&DeploymentStep::BeginMonitoring));
}

#[test]
fn test_runtime_restart() {
    let mut orchestrator = DeploymentOrchestrator::new(sample_manager());
    let result = orchestrator
        .deploy_runtime("arena", "hash", "1.0", vec!["rig".into()])
        .unwrap();
    orchestrator
        .supervisor
        .restart_runtime(&result.runtime.runtime_id)
        .unwrap();
    assert!(orchestrator
        .supervisor
        .verify_runtime(&result.runtime.runtime_id));
}

#[test]
fn test_runtime_upgrade() {
    let mut orchestrator = DeploymentOrchestrator::new(sample_manager());
    let result = orchestrator
        .deploy_runtime("arena", "hash", "1.0", vec!["rig".into()])
        .unwrap();
    orchestrator
        .supervisor
        .upgrade_runtime(&result.runtime.runtime_id, "2.0")
        .unwrap();
    assert_eq!(
        orchestrator.supervisor.runtimes[&result.runtime.runtime_id].version,
        "2.0"
    );
}

#[test]
fn test_runtime_recovery() {
    let mut orchestrator = DeploymentOrchestrator::new(sample_manager());
    let result = orchestrator
        .deploy_runtime("arena", "hash", "1.0", vec!["rig".into()])
        .unwrap();
    orchestrator
        .supervisor
        .recover_runtime(&mut orchestrator.leases, &result.runtime.runtime_id)
        .unwrap();
    assert!(orchestrator.supervisor.runtimes[&result.runtime.runtime_id]
        .recovery_events
        .contains(&"checkpoint restore".to_string()));
}

#[test]
fn test_runtime_health_monitoring() {
    let mut health = RuntimeHealth::healthy("runtime", 10);
    assert_eq!(health.evaluate(), HealthState::Healthy);
}

#[test]
fn test_alert_generation() {
    let mut alerts = AlertManager::default();
    let mut health = RuntimeHealth::healthy("runtime", 10);
    health.runtime_alive = false;
    health.evaluate();
    assert_eq!(
        alerts.evaluate_health(&health)[0].trigger,
        AlertTrigger::NodeLost
    );
}

#[test]
fn test_metrics_collection() {
    let mut metrics = MetricsCollector::default();
    metrics.collect_runtime(60.0, 3.0);
    metrics.record_deployment();
    assert_eq!(metrics.snapshot.runtime.ticks_per_sec, 60.0);
    assert_eq!(metrics.snapshot.deployment.deployment_count, 1);
}

#[test]
fn test_topology_tracking() {
    let topology = FederationTopology::new(5).unwrap();
    assert_eq!(topology.nodes.len(), 5);
    assert_eq!(topology.leader, "node-1");
}

#[test]
fn test_federation_health() {
    assert_eq!(
        FederationTopology::new(10).unwrap().federation_health(),
        "healthy"
    );
}

#[test]
fn test_cost_model_generation() {
    let model = arena_vanguard_cost_model();
    assert_eq!(model.thousand_games.lease_count, 1000);
}

#[test]
fn test_operator_audit() {
    let mut control = OperatorControlPlane::arena_vanguard();
    control
        .orchestrator
        .deploy_runtime("arena", "hash", "1.0", vec!["rig".into()])
        .unwrap();
    assert!(control
        .lease_audit()
        .iter()
        .any(|event| event.contains("allocate lease")));
}

#[test]
fn test_deployment_rollback() {
    let mut orchestrator = DeploymentOrchestrator::new(sample_manager());
    assert!(orchestrator
        .deploy_runtime("arena", "bad", "1.0", vec!["rig".into()])
        .is_err());
    assert_eq!(orchestrator.metrics.snapshot.deployment.rollback_count, 1);
}

#[test]
fn test_arena_vanguard_live_deployment() {
    let path = arena_vanguard_live_deployment_path();
    for step in [
        "Build Package",
        "Validate Package",
        "Allocate Lease",
        "Deploy Runtime",
        "Join Federation",
        "Verify Health",
        "Start Session",
        "Checkpoint",
        "Recover",
        "Upgrade",
        "Rollback",
    ] {
        assert!(path.contains(&step));
    }
}

#[test]
fn test_anchor_payload_generation() {
    let control = OperatorControlPlane::arena_vanguard();
    let intent = control.generate_anchor_intent(AnchorKind::Replay, "replay-root");
    assert!(intent.payload.external_settlement_required);
    assert_eq!(intent.settlement_service, "external-xrpl-settlement");
}

#[test]
fn test_control_plane_recovery() {
    let plan = automated_recovery_plan("runtime");
    assert!(plan.automated);
    assert!(plan.steps.contains(&"replay recovery".to_string()));
}

#[test]
fn test_runtime_supervision() {
    let mut leases = sample_manager();
    let lease = leases.allocate_lease("arena", sample_resources()).unwrap();
    let mut supervisor = RuntimeSupervisor::default();
    supervisor
        .start_runtime(&mut leases, &lease.id, "runtime", "node-1", "1.0")
        .unwrap();
    assert!(supervisor.verify_runtime("runtime"));
}

#[test]
fn test_multi_runtime_management() {
    let mut orchestrator = DeploymentOrchestrator::new(sample_manager());
    for i in 0..3 {
        orchestrator
            .deploy_runtime(
                format!("arena-{i}"),
                format!("hash-{i}"),
                "1.0",
                vec!["rig".into()],
            )
            .unwrap();
    }
    assert_eq!(orchestrator.supervisor.runtimes.len(), 3);
}

#[test]
fn test_authority_boundary_preservation() {
    let control = OperatorControlPlane::arena_vanguard();
    let payload = control.generate_anchor_payload(AnchorKind::Deployment, "deployment-root");
    assert!(payload.external_settlement_required);
    assert_eq!(payload.payload_hash, "deployment-root");
}
