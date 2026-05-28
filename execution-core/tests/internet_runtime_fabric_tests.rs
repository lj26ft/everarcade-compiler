use execution_core::runtime::internet_runtime_fabric::{
    reject_authority_mutation, validate_peer, InternetReplayWindow, InternetRuntimeError,
    InternetRuntimeFabric,
};

const ROOT: &str = "root:everarcade:internet-runtime:v1";

#[test]
fn test_quic_replay_reconnect() {
    let fabric = InternetRuntimeFabric::canonical().expect("canonical fabric");
    let resumed = fabric.quic_resume(192).expect("resumable quic window");
    assert_eq!(resumed.start, 128);
    assert_eq!(resumed.continuity_root, ROOT);
    assert_eq!(
        fabric.quic_resume(128),
        Err(InternetRuntimeError::ReplayInjection)
    );
}

#[test]
fn test_encrypted_peer_transport() {
    let fabric = InternetRuntimeFabric::canonical().expect("canonical fabric");
    assert!(fabric.encrypted_transport);
    assert_eq!(
        validate_peer(
            "peer-a",
            ROOT,
            "peer-sig:peer-a:root:everarcade:internet-runtime:v1"
        ),
        Ok(())
    );
    assert_eq!(
        validate_peer("peer-a", ROOT, "forged"),
        Err(InternetRuntimeError::ForgedPeer)
    );
}

#[test]
fn test_runtime_service_recovery() {
    let fabric = InternetRuntimeFabric::canonical().expect("canonical fabric");
    for service in [
        "federation",
        "network",
        "observer",
        "replay",
        "storage",
        "supervisor",
        "watchdog",
        "recovery",
    ] {
        assert!(fabric.services.contains(service));
    }
    assert_eq!(fabric.report().replay_continuity, "ok");
}

#[test]
fn test_persistent_topology_restoration() {
    let mut fabric = InternetRuntimeFabric::canonical().expect("canonical fabric");
    let snapshot = fabric
        .persist_topology_snapshot()
        .expect("topology snapshot");
    fabric.topology.clear();
    fabric
        .restore_topology_snapshot(&snapshot)
        .expect("restore snapshot");
    assert_eq!(fabric.topology.len(), 3);
    assert_eq!(fabric.report().topology_state, "restorable");
}

#[test]
fn test_replay_compression_equivalence() {
    let fabric = InternetRuntimeFabric::canonical().expect("canonical fabric");
    let compressed = fabric.compress_windows().expect("compressed windows");
    assert_eq!(compressed.len(), fabric.windows.len());
    assert!(compressed[0].starts_with("0..64:"));
}

#[test]
fn test_incremental_observer_hydration() {
    let mut fabric = InternetRuntimeFabric::canonical().expect("canonical fabric");
    let hydrated = fabric
        .hydrate_observer("observer-us-east", 64)
        .expect("incremental hydration");
    assert_eq!(hydrated.len(), 1);
    assert_eq!(fabric.report().observer_state, "hydrated");
}

#[test]
fn test_runtime_autoscaling_equivalence() {
    let mut left = InternetRuntimeFabric::canonical().expect("left fabric");
    let mut right = InternetRuntimeFabric::canonical().expect("right fabric");
    left.autoscale(5).expect("left autoscale");
    right.autoscale(5).expect("right autoscale");
    assert_eq!(
        left.persist_topology_snapshot(),
        right.persist_topology_snapshot()
    );
}

#[test]
fn test_global_observer_replay_distribution() {
    let mut fabric = InternetRuntimeFabric::canonical().expect("canonical fabric");
    let europe = fabric
        .hydrate_observer("observer-eu", 0)
        .expect("eu hydration");
    let asia = fabric
        .hydrate_observer("observer-apac", 0)
        .expect("apac hydration");
    assert_eq!(europe, asia);
    assert!(fabric.reconstruction_only);
}

#[test]
fn test_storage_fabric_restoration() {
    let fabric = InternetRuntimeFabric::canonical().expect("canonical fabric");
    let archived = fabric.compress_windows().expect("archive windows");
    let replayed = fabric.compress_windows().expect("restore windows");
    assert_eq!(archived, replayed);
}

#[test]
fn test_wan_recovery_equivalence() {
    let fabric = InternetRuntimeFabric::canonical().expect("canonical fabric");
    let recovered = fabric.quic_resume(256).expect("wan stream resume");
    assert_eq!(recovered.start, 128);
    assert_eq!(recovered.end, 256);
}

#[test]
fn test_runtime_fabric_integrity() {
    let mut fabric = InternetRuntimeFabric::canonical().expect("canonical fabric");
    assert_eq!(
        fabric.append_window(InternetReplayWindow {
            start: 128,
            end: 192,
            continuity_root: ROOT.into(),
            payload_hash: "tampered".into()
        }),
        Err(InternetRuntimeError::CorruptedReplay)
    );
    assert_eq!(
        reject_authority_mutation("mutable_authority_state"),
        Err(InternetRuntimeError::AuthorityMutation)
    );
}

#[test]
fn test_runtime_non_authoritative() {
    let fabric = InternetRuntimeFabric::canonical().expect("canonical fabric");
    let report = fabric.report();
    assert!(report.reconstruction_only);
    assert!(!report.renderer_authoritative);
    assert_eq!(reject_authority_mutation("replay-window"), Ok(()));
}
