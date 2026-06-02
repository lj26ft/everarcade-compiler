use control_plane::leases::LeaseResources;
use control_plane::provider::{ProviderLeaseRequest, ProviderPackage, RuntimeProvider};
use provider_evernode::{
    rollback_to_checkpoint, upgrade_runtime, EverNodeProvider, EverNodeTopology, RecoveryRoots,
};

fn deploy(nodes: usize) -> EverNodeProvider {
    let mut topology = EverNodeTopology::new(nodes).unwrap();
    topology.validate_all();
    let mut provider = EverNodeProvider::new(format!("evernode-host-{nodes}"));
    let lease = provider
        .allocate_lease(ProviderLeaseRequest {
            game_id: "arena-vanguard".into(),
            resources: LeaseResources {
                cpu_cores: 4,
                memory_mb: 8192,
                storage_gb: 200,
                bandwidth_mbps: 100,
            },
        })
        .unwrap();
    provider
        .deploy_package(
            &lease.lease_id,
            ProviderPackage {
                game_id: "arena-vanguard".into(),
                package_hash: format!("hash-{nodes}"),
                runtime_version: "hotpocket-0.6".into(),
                rustrig_hashes: vec!["rustrig-hash".into()],
            },
        )
        .unwrap();
    provider.metrics.snapshot.federation.node_count = nodes;
    provider
}

#[test]
fn single_node_deployment() {
    assert_eq!(
        deploy(1).collect_metrics().unwrap().federation.node_count,
        1
    );
}

#[test]
fn two_node_deployment() {
    assert_eq!(
        deploy(2).collect_metrics().unwrap().federation.node_count,
        2
    );
}

#[test]
fn five_node_deployment() {
    assert_eq!(
        deploy(5).collect_metrics().unwrap().federation.node_count,
        5
    );
}

#[test]
fn ten_node_deployment() {
    assert_eq!(
        deploy(10).collect_metrics().unwrap().federation.node_count,
        10
    );
}

#[test]
fn failure_recovery() {
    let mut provider = deploy(2);
    let runtime_id = provider.runtimes.keys().next().unwrap().clone();
    provider.stop_runtime(&runtime_id).unwrap();
    provider.restart_runtime(&runtime_id).unwrap();
    assert!(
        provider
            .perform_recovery(&runtime_id)
            .unwrap()
            .rejoined_federation
    );
}

#[test]
fn upgrade_recovery() {
    assert!(upgrade_runtime("upgrade-package").unwrap().traffic_resumed);
}

#[test]
fn rollback_recovery() {
    assert!(
        rollback_to_checkpoint("checkpoint", &RecoveryRoots::sample())
            .unwrap()
            .federation_resumed
    );
}
