use control_plane::leases::LeaseResources;
use control_plane::provider::{ProviderLeaseRequest, ProviderPackage, RuntimeProvider};
use provider_evernode::{
    rollback_available, rollback_to_checkpoint, upgrade_runtime, EverNodeProvider,
    EverNodeTopology, HotPocketStateLayout, ProviderMetricExports, RecoveryRoots,
};

fn resources() -> LeaseResources {
    LeaseResources {
        cpu_cores: 4,
        memory_mb: 8192,
        storage_gb: 200,
        bandwidth_mbps: 100,
    }
}

fn package() -> ProviderPackage {
    ProviderPackage {
        game_id: "arena-vanguard".into(),
        package_hash: "hash-arena-vanguard".into(),
        runtime_version: "hotpocket-0.6".into(),
        rustrig_hashes: vec!["rustrig-hash".into()],
    }
}

fn deployed_provider() -> (EverNodeProvider, String, String) {
    let mut provider = EverNodeProvider::default();
    let lease = provider
        .allocate_lease(ProviderLeaseRequest {
            game_id: "arena-vanguard".into(),
            resources: resources(),
        })
        .unwrap();
    let deployment = provider.deploy_package(&lease.lease_id, package()).unwrap();
    (provider, lease.lease_id, deployment.runtime_id)
}

#[test]
fn test_provider_lease_allocation() {
    let mut provider = EverNodeProvider::default();
    let lease = provider
        .allocate_lease(ProviderLeaseRequest {
            game_id: "arena-vanguard".into(),
            resources: resources(),
        })
        .unwrap();
    assert_eq!(lease.state, "Allocated");
    assert!(provider.leases[&lease.lease_id]
        .audit()
        .contains(&"lease allocation".to_string()));
}

#[test]
fn test_provider_deployment() {
    let (_provider, _lease_id, runtime_id) = deployed_provider();
    assert!(runtime_id.starts_with("runtime-evernode-lease"));
}

#[test]
fn test_provider_runtime_start() {
    let (mut provider, _lease_id, runtime_id) = deployed_provider();
    assert_eq!(
        provider.start_runtime(&runtime_id).unwrap().process_state,
        "running"
    );
}

#[test]
fn test_provider_runtime_stop() {
    let (mut provider, _lease_id, runtime_id) = deployed_provider();
    assert_eq!(
        provider.stop_runtime(&runtime_id).unwrap().process_state,
        "stopped"
    );
}

#[test]
fn test_provider_runtime_restart() {
    let (mut provider, _lease_id, runtime_id) = deployed_provider();
    provider.stop_runtime(&runtime_id).unwrap();
    assert_eq!(
        provider.restart_runtime(&runtime_id).unwrap().process_state,
        "running"
    );
}

#[test]
fn test_provider_health_collection() {
    let (provider, _lease_id, runtime_id) = deployed_provider();
    let health = provider.collect_health(&runtime_id).unwrap();
    assert!(health.runtime_alive);
    assert_eq!(health.network_status, "federated");
}

#[test]
fn test_provider_metrics_collection() {
    let (provider, _lease_id, _runtime_id) = deployed_provider();
    let metrics = provider.collect_metrics().unwrap();
    let exports = ProviderMetricExports::from_snapshot(&metrics, provider.leases.len(), 1, 1);
    assert_eq!(exports.deployment_metrics, 1);
    assert_eq!(exports.lease_metrics, 1);
}

#[test]
fn test_provider_recovery() {
    let (mut provider, _lease_id, runtime_id) = deployed_provider();
    let recovery = provider.perform_recovery(&runtime_id).unwrap();
    assert!(recovery.rejoined_federation);
    assert_eq!(recovery.continuity_root, "continuity-root");
}

#[test]
fn test_provider_upgrade() {
    let report = upgrade_runtime("new-package-hash").unwrap();
    assert!(report.checkpoint_created && report.convergence_verified && report.rollback_available);
}

#[test]
fn test_provider_rollback() {
    let roots = RecoveryRoots::sample();
    let report = rollback_to_checkpoint("checkpoint-1", &roots).unwrap();
    assert!(report.restored_state && report.verified_replay_root && report.federation_resumed);
}

#[test]
fn test_provider_federation_join() {
    let mut topology = EverNodeTopology::new(5).unwrap();
    topology.validate_all();
    assert!(topology.has_validation("node join"));
}

#[test]
fn test_provider_checkpoint_sync() {
    let mut topology = EverNodeTopology::new(2).unwrap();
    topology.validate_all();
    assert!(topology.has_validation("checkpoint sync"));
}

#[test]
fn test_provider_replay_sync() {
    let mut topology = EverNodeTopology::new(10).unwrap();
    topology.validate_all();
    assert!(topology.has_validation("replay sync"));
}

#[test]
fn test_provider_partition_recovery() {
    let mut topology = EverNodeTopology::new(10).unwrap();
    topology.validate_all();
    assert!(topology.has_validation("partition recovery"));
}

#[test]
fn test_arena_vanguard_live_deployment() {
    let (mut provider, _lease_id, runtime_id) = deployed_provider();
    assert!(provider.collect_health(&runtime_id).unwrap().runtime_alive);
    assert!(
        provider
            .perform_recovery(&runtime_id)
            .unwrap()
            .rejoined_federation
    );
    let upgrade = upgrade_runtime("arena-vanguard-v2").unwrap();
    let rollback = rollback_to_checkpoint("arena-checkpoint", &RecoveryRoots::sample()).unwrap();
    assert!(rollback_available(&upgrade, &rollback));
}

#[test]
fn test_anchor_payload_generation() {
    let provider = EverNodeProvider::default();
    assert!(provider.host.is_ready());
    // Provider packages anchor payload material only; XRPL/Xahau signing and submission are external.
    assert!(!provider.logs.export_json().contains("signing_secret"));
}

#[test]
fn test_authority_boundary_preservation() {
    let layout = HotPocketStateLayout::canonical("state");
    layout.validate().unwrap();
    assert!(layout
        .validate_authority_write("state/world/world.bin")
        .is_ok());
    assert!(layout.validate_authority_write("xrpl/signature").is_err());
    assert!(layout
        .validate_authority_write("state/unknown/file")
        .is_err());
}
