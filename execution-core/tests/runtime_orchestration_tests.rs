use execution_core::runtime::orchestration_scaling::{
    canonical_fixture, deployment_manifest, hydrate_observers, package_bundle_root,
    quic_resume_window, recover_stalled_stream, throttle_capacity, trusted_peer,
    validate_peer_signature, ReplayPeer, ReplayWindow, RuntimeFederationError,
    RuntimeFederationOrchestrator,
};

const ROOT: &str = "root:everarcade:federation:v1";

#[test]
fn test_multinode_replay_propagation() {
    let orchestrator = canonical_fixture().expect("canonical runtime federation fixture");
    let report = orchestrator.report(3);
    assert_eq!(report.node_count, 3);
    assert!(report.replay_only);
    assert!(report.ordering_preserved);
}

#[test]
fn test_runtime_topology_recovery() {
    let mut orchestrator = canonical_fixture().expect("canonical runtime federation fixture");
    orchestrator
        .restore_topology()
        .expect("topology restoration");
    assert!(orchestrator.report(3).recovery_ready);
}

#[test]
fn test_runtime_peer_pool_scaling() {
    let mut orchestrator = canonical_fixture().expect("canonical runtime federation fixture");
    orchestrator
        .register_peer(trusted_peer("peer-c", "lineage-c", ROOT))
        .expect("trusted peer accepted");
    assert_eq!(orchestrator.report(3).peer_count, 3);
}

#[test]
fn test_runtime_shard_routing_equivalence() {
    let orchestrator = canonical_fixture().expect("canonical runtime federation fixture");
    let routes = orchestrator
        .route_shards(6)
        .expect("deterministic shard routes");
    assert_eq!(routes.len(), 6);
    assert_eq!(routes[0].window.continuity_root, ROOT);
    assert_eq!(routes[0].node_id, routes[3].node_id);
}

#[test]
fn test_runtime_backpressure_recovery() {
    assert_eq!(throttle_capacity(4, 10).expect("capacity"), 6);
    assert_eq!(
        throttle_capacity(11, 10),
        Err(RuntimeFederationError::ReplayCorruption)
    );
}

#[test]
fn test_runtime_quic_replay_transport() {
    let previous = ReplayWindow {
        start: 0,
        end: 64,
        continuity_root: ROOT.to_string(),
    };
    let resumed = quic_resume_window(&previous, 96).expect("quic resume window");
    assert_eq!(resumed.start, 64);
    assert_eq!(resumed.continuity_root, ROOT);
}

#[test]
fn test_runtime_authenticated_peer_rejection() {
    let forged = ReplayPeer {
        id: "peer-x".to_string(),
        lineage_owner: "lineage-x".to_string(),
        signature: "forged".to_string(),
        trusted: true,
    };
    assert!(!validate_peer_signature(&forged, ROOT));
    let mut orchestrator = RuntimeFederationOrchestrator::new(ROOT);
    assert_eq!(
        orchestrator.register_peer(forged),
        Err(RuntimeFederationError::UnauthorizedPeer)
    );
}

#[test]
fn test_runtime_observer_hydration_scaling() {
    let windows = vec![
        ReplayWindow {
            start: 0,
            end: 10,
            continuity_root: ROOT.to_string(),
        },
        ReplayWindow {
            start: 10,
            end: 20,
            continuity_root: ROOT.to_string(),
        },
        ReplayWindow {
            start: 20,
            end: 30,
            continuity_root: ROOT.to_string(),
        },
    ];
    let hydrated = hydrate_observers(2, &windows).expect("observer hydration partitions");
    assert_eq!(hydrated.len(), 2);
    assert_eq!(hydrated[&0].len(), 2);
}

#[test]
fn test_runtime_deployment_restoration() {
    let orchestrator = canonical_fixture().expect("canonical runtime federation fixture");
    let routes = orchestrator.route_shards(3).expect("routes");
    assert!(routes
        .iter()
        .all(|route| route.window.continuity_root == ROOT));
}

#[test]
fn test_runtime_orchestrator_equivalence() {
    let left = canonical_fixture().expect("left fixture").report(4);
    let right = canonical_fixture().expect("right fixture").report(4);
    assert_eq!(left, right);
}

#[test]
fn test_runtime_replay_federation_integrity() {
    let orchestrator = canonical_fixture().expect("canonical runtime federation fixture");
    let latest = recover_stalled_stream(&[
        ReplayWindow {
            start: 0,
            end: 64,
            continuity_root: ROOT.to_string(),
        },
        ReplayWindow {
            start: 64,
            end: 128,
            continuity_root: ROOT.to_string(),
        },
    ])
    .expect("ordered replay recovery");
    assert_eq!(latest.end, 128);
    assert_eq!(
        package_bundle_root(&["runtime", "network", "storage"]).expect("package root"),
        "evernode-bundle:network+runtime+storage"
    );
    assert_eq!(orchestrator.report(3).continuity_root, ROOT);
}

#[test]
fn test_runtime_non_authoritative() {
    let orchestrator = canonical_fixture().expect("canonical runtime federation fixture");
    let report = orchestrator.report(3);
    assert!(!report.renderer_authoritative);
    assert!(report.replay_only);
}

#[test]
fn test_runtime_deployment_manifest_preserves_continuity() {
    let mut orchestrator = RuntimeFederationOrchestrator::new(ROOT);
    orchestrator.register_node("node-a").expect("node a");
    orchestrator.register_node("node-b").expect("node b");
    let routes = orchestrator.report(0);
    assert_eq!(routes.continuity_root, ROOT);
    let nodes = vec![
        execution_core::runtime::orchestration_scaling::RuntimeNode {
            id: "node-a".into(),
            continuity_root: ROOT.into(),
            online: true,
        },
        execution_core::runtime::orchestration_scaling::RuntimeNode {
            id: "node-b".into(),
            continuity_root: ROOT.into(),
            online: true,
        },
    ];
    let manifest = deployment_manifest(&nodes, ROOT).expect("deployment manifest");
    assert!(manifest.contains("continuity_root=root:everarcade:federation:v1"));
}
