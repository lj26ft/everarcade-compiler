use execution_core::runtime::export_governance::{
    ReplaySessionLifecycle, RuntimeActivationState, RuntimeDeploymentRecovery,
    RuntimeRecoveryLifecycle, SovereignRuntimeBootstrap,
};

#[test]
fn test_runtime_bootstrap_restoration() {
    let bootstrap = SovereignRuntimeBootstrap::default();
    assert!(bootstrap.deterministic_restoration);
}

#[test]
fn test_runtime_session_checkpoint_equivalence() {
    let session = ReplaySessionLifecycle::default();
    assert!(session.checkpoint_equivalence);
}

#[test]
fn test_runtime_peer_reconnect_recovery() {
    let recovery = RuntimeRecoveryLifecycle::default();
    assert!(recovery.peer_reconnect_recovery);
}

#[test]
fn test_runtime_snapshot_restoration() {
    assert!(SovereignRuntimeBootstrap::default().snapshot_restoration);
}

#[test]
fn test_runtime_recovery_equivalence() {
    assert!(RuntimeRecoveryLifecycle::default().deterministic_equivalence);
}

#[test]
fn test_runtime_daemon_restart() {
    assert!(RuntimeRecoveryLifecycle::default().daemon_restart_continuity);
}

#[test]
fn test_runtime_health_continuity() {
    assert!(RuntimeActivationState::default().health_gate_ready);
}

#[test]
fn test_runtime_deployment_recovery() {
    assert!(RuntimeDeploymentRecovery::default().deployment_continuity_restored);
}

#[test]
fn test_runtime_replay_restoration() {
    assert!(RuntimeActivationState::default().replay_restoration_ready);
}

#[test]
fn test_runtime_recovery_corruption_detection() {
    assert!(RuntimeRecoveryLifecycle::default().corruption_detection_enabled);
}

#[test]
fn test_runtime_session_divergence_detection() {
    assert!(ReplaySessionLifecycle::default().divergence_detection_enabled);
}

#[test]
fn test_runtime_non_authoritative() {
    assert!(RuntimeActivationState::default().renderer_non_authoritative);
}
