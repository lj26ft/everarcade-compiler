use everarcade_host::federation_transport::network::*;

fn mk(peer: &str, checkpoint: u64) -> PeerSession {
    let mut s = start_peer_session(peer.to_string(), 1, "lease-soak".to_string());
    s.continuity_checkpoint = checkpoint;
    s
}

#[test]
fn test_autonomous_session_recovery() {
    let mut c = SessionCoordinator::default();
    coordinate_sessions(&mut c, vec![mk("a", 1)]);
    let mut sup = start_runtime_supervisor(&c);
    assert!(recover_runtime_session(&mut c, &mut sup, "a"));
    assert!(resume_autonomous_sync(&mut c, &mut sup, "a"));
}

#[test]
fn test_automatic_divergence_resolution() {
    let mut c = SessionCoordinator::default();
    coordinate_sessions(&mut c, vec![mk("a", 3)]);
    let mut sup = start_runtime_supervisor(&c);
    assert!(detect_live_divergence_event(&mut sup, false));
    assert!(recover_live_continuity(&mut c, &mut sup, "a", 4));
}

#[test]
fn test_runtime_supervisor_recovers_failed_peer() {
    let mut c = SessionCoordinator::default();
    coordinate_sessions(&mut c, vec![mk("peer-1", 2)]);
    let mut sup = start_runtime_supervisor(&c);
    c.active_sessions.get_mut("peer-1").unwrap().connected = false;
    assert!(detect_runtime_failure(&c, &sup, "peer-1"));
    assert!(coordinate_runtime_recovery(&mut c, &mut sup, "peer-1"));
}

#[test]
fn test_live_replay_verification_after_restart() {
    assert!(verify_live_replay_continuity(true, true, true));
    assert!(verify_continuous_integrity(true, true, true, true));
}

#[test]
fn test_long_term_state_root_convergence() {
    let mut c = SessionCoordinator::default();
    coordinate_sessions(&mut c, vec![mk("a", 0)]);
    let mut sup = start_runtime_supervisor(&c);
    for cp in 1..=1000 {
        assert!(coordinate_incremental_advancement(
            &mut c, &mut sup, "a", cp
        ));
    }
    assert!(verify_live_state_root_convergence([9u8; 32], [9u8; 32]));
    assert!(verify_supervisor_state(&sup));
}
