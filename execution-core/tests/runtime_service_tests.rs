use execution_core::runtime::operational_hardening::{
    LiveTransportBoundaryRuntime, ObserverStreamRuntime, PeerSessionRuntime,
    ReplayPersistenceRuntime, ReplayStorageEngineRuntime, RuntimeAdversarialValidation,
    RuntimeDeploymentRestorationRuntime, RuntimeNodeServiceRuntime, RuntimeServiceHealthRuntime,
};

#[test]
fn test_runtime_service_loop_equivalence() {
    let mut first = RuntimeNodeServiceRuntime::start("node-a", "replay-root");
    let mut second = RuntimeNodeServiceRuntime::start("node-a", "replay-root");
    RuntimeNodeServiceRuntime::process_cycles(&mut first, 4);
    RuntimeNodeServiceRuntime::process_cycles(&mut second, 4);
    assert!(RuntimeNodeServiceRuntime::equivalent(&first, &second));
}

#[test]
fn test_runtime_service_checkpoint_restore() {
    let mut state = RuntimeNodeServiceRuntime::start("node-b", "replay-root");
    RuntimeNodeServiceRuntime::process_cycles(&mut state, 3);
    let checkpoint = RuntimeNodeServiceRuntime::checkpoint(&state);
    let restored = RuntimeNodeServiceRuntime::restore(&checkpoint).unwrap();
    assert!(RuntimeNodeServiceRuntime::equivalent(&state, &restored));
}

#[test]
fn test_runtime_storage_engine_restore() {
    let mut storage = ReplayStorageEngineRuntime::default();
    let checkpoint =
        ReplayPersistenceRuntime::persist_checkpoint("cp-1", 1, "replay", "continuity");
    storage.append(checkpoint.clone()).unwrap();
    assert_eq!(storage.restore_latest().unwrap(), checkpoint);
}

#[test]
fn test_runtime_storage_compaction() {
    let mut storage = ReplayStorageEngineRuntime::default();
    storage
        .append(ReplayPersistenceRuntime::persist_checkpoint(
            "cp-1", 1, "replay", "c1",
        ))
        .unwrap();
    storage
        .append(ReplayPersistenceRuntime::persist_checkpoint(
            "cp-2", 2, "replay", "c2",
        ))
        .unwrap();
    assert_eq!(storage.compact().unwrap().checkpoint_id, "cp-2");
}

#[test]
fn test_runtime_peer_session_recovery() {
    let mut session = PeerSessionRuntime::establish("peer-a", "continuity");
    PeerSessionRuntime::synchronize_window(&mut session, 8);
    let recovered = PeerSessionRuntime::recover(&session);
    PeerSessionRuntime::validate(&session, &recovered).unwrap();
}

#[test]
fn test_runtime_transport_boundary_equivalence() {
    let mut left = LiveTransportBoundaryRuntime::default();
    let mut right = LiveTransportBoundaryRuntime::default();
    for sequence in 0..3 {
        let chunk =
            LiveTransportBoundaryRuntime::chunk(sequence, &format!("payload-{sequence}"), "root");
        left.ingest(chunk.clone()).unwrap();
        right.ingest(chunk).unwrap();
    }
    assert_eq!(left, right);
}

#[test]
fn test_runtime_observer_stream_restore() {
    let stream = ObserverStreamRuntime::stream("observer-a", 3, "continuity");
    let restored = ObserverStreamRuntime::restore(&stream).unwrap();
    assert_eq!(restored, stream);
    assert!(restored.non_authoritative);
}

#[test]
fn test_runtime_service_restart_recovery() {
    let mut state = RuntimeNodeServiceRuntime::start("node-c", "replay-root");
    RuntimeNodeServiceRuntime::process_cycles(&mut state, 5);
    let checkpoint = RuntimeNodeServiceRuntime::checkpoint(&state);
    let restarted = RuntimeNodeServiceRuntime::restore(&checkpoint).unwrap();
    assert!(restarted.running);
    assert!(RuntimeNodeServiceRuntime::equivalent(&state, &restarted));
}

#[test]
fn test_runtime_health_gate_equivalence() {
    let health = RuntimeServiceHealthRuntime::evaluate("continuity");
    RuntimeServiceHealthRuntime::gate(&health).unwrap();
    assert_eq!(health, RuntimeServiceHealthRuntime::evaluate("continuity"));
}

#[test]
fn test_runtime_deployment_restoration() {
    let deployment =
        RuntimeDeploymentRestorationRuntime::capture("deploy-a", "topology", "continuity");
    let restored = RuntimeDeploymentRestorationRuntime::restore(&deployment).unwrap();
    assert_eq!(deployment, restored);
}

#[test]
fn test_runtime_replay_corruption_rejection() {
    let checkpoint =
        ReplayPersistenceRuntime::persist_checkpoint("cp-corrupt", 9, "replay", "continuity");
    assert!(RuntimeAdversarialValidation::reject_replay_corruption(
        checkpoint
    ));

    let mut service = RuntimeNodeServiceRuntime::start("node-d", "replay-root");
    RuntimeNodeServiceRuntime::process_cycles(&mut service, 1);
    assert!(RuntimeAdversarialValidation::reject_checkpoint_corruption(
        RuntimeNodeServiceRuntime::checkpoint(&service)
    ));
    assert!(RuntimeAdversarialValidation::reject_transport_duplication());
}

#[test]
fn test_runtime_non_authoritative() {
    let state = RuntimeNodeServiceRuntime::start("node-e", "replay-root");
    let observer = ObserverStreamRuntime::stream("observer-b", 1, "continuity");
    assert!(state.reconstruction_only);
    assert!(observer.non_authoritative);
}
