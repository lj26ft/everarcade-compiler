use everarcade_host::federation_transport::network::*;

fn mk_session(peer: &str, checkpoint: u64) -> PeerSession {
    let mut s = start_peer_session(peer.to_string(), 1, "lease-c".to_string());
    s.continuity_checkpoint = checkpoint;
    s
}

#[test]
fn test_long_running_incremental_sync() {
    let mut c = SessionCoordinator::default();
    coordinate_sessions(
        &mut c,
        vec![mk_session("a", 1), mk_session("b", 1), mk_session("c", 1)],
    );
    assert!(advance_runtime_sync(&mut c, "a", 2));
    assert_eq!(inspect_runtime_advancement(&c, "a"), Some(2));
}

#[test]
fn test_continuous_checkpoint_advancement() {
    let mut c = SessionCoordinator::default();
    coordinate_sessions(&mut c, vec![mk_session("a", 3)]);
    assert!(resume_checkpoint_advancement(&mut c, "a", 4));
    assert!(resume_checkpoint_advancement(&mut c, "a", 5));
}

#[test]
fn test_repeated_restart_recovery() {
    let mut c = SessionCoordinator::default();
    coordinate_sessions(&mut c, vec![mk_session("a", 5)]);
    assert!(recover_distributed_runtime(&mut c));
    assert!(verify_runtime_recovery_state(&c, "a"));
}

#[test]
fn test_continuous_replay_verification() {
    assert!(verify_live_replay(true, true, true));
    assert!(verify_continuous_convergence(true, true));
    assert!(detect_live_divergence(false));
}

#[test]
fn test_sustained_state_root_convergence() {
    assert!(restore_checkpoint_lineage([7u8; 32], [7u8; 32]));
    assert!(reject_invalid_live_continuity(true, false, false));
}
