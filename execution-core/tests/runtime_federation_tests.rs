use execution_core::runtime::operational_hardening::{
    DistributedReplayTransportRuntime, FederationRoutingRuntime, FederationTopologyRuntime,
    ReplayStreamPropagationRuntime, RuntimeFederationHealthRuntime, RuntimeFederationRuntime,
};

#[test]
fn test_live_peer_handshake_equivalence() {
    let left = RuntimeFederationRuntime::handshake("peer-a", "topology", "continuity");
    let right = RuntimeFederationRuntime::handshake("peer-a", "topology", "continuity");
    RuntimeFederationRuntime::validate_equivalence(&left, &right).unwrap();
}

#[test]
fn test_distributed_replay_chunk_propagation() {
    let mut transport = DistributedReplayTransportRuntime::default();
    let frame = DistributedReplayTransportRuntime::frame(0, "replay-a", "continuity-a");
    transport.propagate(frame).unwrap();
    assert!(transport.continuity_root().is_some());
}

#[test]
fn test_federation_topology_recovery() {
    let topology = FederationTopologyRuntime::topology(&["peer-b", "peer-a"], "continuity");
    let recovered = FederationTopologyRuntime::recover(&topology).unwrap();
    assert_eq!(topology, recovered);
}

#[test]
fn test_runtime_peer_reconnect() {
    let mut peer = RuntimeFederationRuntime::handshake("peer-a", "topology", "continuity");
    RuntimeFederationRuntime::synchronize_window(&mut peer, 7).unwrap();
    let reconnected = RuntimeFederationRuntime::reconnect(&peer).unwrap();
    assert_eq!(peer, reconnected);
}

#[test]
fn test_runtime_stream_recovery() {
    let stream = ReplayStreamPropagationRuntime::propagate("stream-a", 3, "continuity");
    let restored = ReplayStreamPropagationRuntime::restore(&stream).unwrap();
    assert_eq!(stream, restored);
}

#[test]
fn test_runtime_checkpoint_distribution() {
    let mut peer = RuntimeFederationRuntime::handshake("peer-a", "topology", "continuity");
    RuntimeFederationRuntime::synchronize_window(&mut peer, 4).unwrap();
    let checkpoint = RuntimeFederationRuntime::checkpoint(&peer, "checkpoint-a");
    let restored = RuntimeFederationRuntime::restore_checkpoint(&checkpoint).unwrap();
    assert_eq!(checkpoint, restored);
}

#[test]
fn test_runtime_peer_divergence_detection() {
    let left = RuntimeFederationRuntime::handshake("peer-a", "topology", "continuity");
    let mut right = left.clone();
    right.continuity_root.push_str("-diverged");
    assert!(RuntimeFederationRuntime::validate_equivalence(&left, &right).is_err());
}

#[test]
fn test_runtime_transport_corruption_detection() {
    let mut transport = DistributedReplayTransportRuntime::default();
    let mut frame = DistributedReplayTransportRuntime::frame(0, "replay-a", "continuity-a");
    frame.frame_root.push_str("-corrupt");
    assert!(transport.propagate(frame).is_err());
}

#[test]
fn test_runtime_federation_health_equivalence() {
    let health = RuntimeFederationHealthRuntime::evaluate("continuity");
    RuntimeFederationHealthRuntime::gate(&health).unwrap();
    assert_eq!(
        health,
        RuntimeFederationHealthRuntime::evaluate("continuity")
    );
}

#[test]
fn test_runtime_federation_deployment_restoration() {
    let topology = FederationTopologyRuntime::topology(&["peer-a", "peer-b"], "continuity");
    let recovered = FederationTopologyRuntime::recover(&topology).unwrap();
    let mut peer =
        RuntimeFederationRuntime::handshake("peer-a", &recovered.topology_root, "continuity");
    RuntimeFederationRuntime::synchronize_window(&mut peer, 2).unwrap();
    RuntimeFederationRuntime::validate_peer_lineage(&peer).unwrap();
}

#[test]
fn test_runtime_replay_lineage_equivalence() {
    let mut routes = FederationRoutingRuntime::default();
    let route = FederationRoutingRuntime::route("window", "peer-a", "peer-b", 0, "continuity");
    routes.accept(route).unwrap();
    assert_eq!(routes.routes.len(), 1);
}

#[test]
fn test_runtime_non_authoritative() {
    let peer = RuntimeFederationRuntime::handshake("peer-a", "topology", "continuity");
    let stream = ReplayStreamPropagationRuntime::propagate("stream-a", 1, "continuity");
    let topology = FederationTopologyRuntime::topology(&["peer-a"], "continuity");
    assert!(peer.reconstruction_only);
    assert!(stream.reconstruction_only);
    assert!(topology.reconstruction_only);
}
