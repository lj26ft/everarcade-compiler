use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use execution_core::certification::cross_machine::{
    CrossMachineSession, CrossMachineTransport, CrossMachineTransportMode,
};
use tempfile::TempDir;

fn make_session() -> (TempDir, TempDir, CrossMachineSession) {
    let machine_a = TempDir::new().expect("machine a root");
    let machine_b = TempDir::new().expect("machine b root");
    let session = CrossMachineSession::boot(machine_a.path(), machine_b.path())
        .expect("cross-machine session boots");
    (machine_a, machine_b, session)
}

#[test]
fn test_cross_machine_join() {
    let (_a_root, _b_root, session) = make_session();
    assert!(session.machine_a.has_independent_roots(&session.machine_b));
    assert_ne!(session.machine_a.process_id, session.machine_b.process_id);
    assert_ne!(
        session.machine_a.runtime_root,
        session.machine_b.runtime_root
    );
    assert_ne!(
        session.machine_a.storage_root,
        session.machine_b.storage_root
    );
    assert_eq!(
        session.machine_a.runtime.world_root(),
        session.machine_b.runtime.world_root()
    );
}

#[test]
fn test_cross_machine_convergence() {
    for ticks in [1_000, 5_000, 10_000] {
        let (_a_root, _b_root, mut session) = make_session();
        let convergence = session.run_convergence(ticks).expect("ticks converge");
        assert!(convergence.converged(), "{ticks} ticks should converge");
        assert_eq!(
            session.machine_a.runtime.world_root(),
            session.machine_b.runtime.world_root()
        );
        assert_eq!(
            session.machine_a.runtime.replay_root(),
            session.machine_b.runtime.replay_root()
        );
        assert_eq!(
            session.machine_a.runtime.checkpoint().checkpoint_root,
            session.machine_b.runtime.checkpoint().checkpoint_root
        );
        assert_eq!(
            session.machine_a.runtime.continuity_root(),
            session.machine_b.runtime.continuity_root()
        );
    }
}

#[test]
fn test_cross_machine_replay_sync() {
    let (_a_root, _b_root, mut session) = make_session();
    session.run_convergence(128).unwrap();
    session.synchronize_replay().expect("replay sync over tcp");
    assert_eq!(
        session.machine_a.runtime.replay_root(),
        session.machine_b.runtime.replay_root()
    );
    assert_eq!(
        session.machine_a.runtime.replay_hash(),
        session.machine_b.runtime.replay_hash()
    );
    assert_eq!(
        session.machine_a.runtime.continuity_root(),
        session.machine_b.runtime.continuity_root()
    );
}

#[test]
fn test_cross_machine_checkpoint_transfer() {
    let (_a_root, _b_root, mut session) = make_session();
    session.run_convergence(256).unwrap();
    let transferred = session
        .transfer_checkpoint()
        .expect("checkpoint transfer over tcp");
    assert_eq!(
        transferred.checkpoint_hash,
        session.machine_b.runtime.checkpoint().checkpoint_root
    );
    assert_eq!(
        transferred.checkpoint.world_root,
        session.machine_b.runtime.world_root()
    );
}

#[test]
fn test_cross_machine_failure_survival() {
    let (_a_root, _b_root, mut session) = make_session();
    session.run_convergence(32).unwrap();
    session
        .fail_machine_a_and_survive(16)
        .expect("machine b survives failover");
    assert!(!session.machine_a.runtime.session.active);
    assert!(session.machine_b.runtime.session.active);
    assert!(!session.machine_b.runtime.world_root().is_empty());
    assert!(!session.machine_b.runtime.replay_root().is_empty());
}

#[test]
fn test_cross_machine_recovery() {
    let (_a_root, _b_root, mut session) = make_session();
    session.run_convergence(32).unwrap();
    session.fail_machine_a_and_survive(16).unwrap();
    let recovery = session
        .recover_machine_a()
        .expect("machine a recovers from machine b");
    assert!(recovery.recovery.restored);
    assert!(recovery.checkpoint_synchronized);
    assert!(recovery.replay_synchronized);
    assert!(recovery.continuity_restored);
    assert!(session
        .machine_a
        .runtime
        .require_convergence(&session.machine_b.runtime)
        .unwrap()
        .converged());
}

#[test]
fn test_cross_machine_transport_interruption() {
    let (_a_root, _b_root, mut session) = make_session();
    session
        .certify_transport()
        .expect("tcp transport surfaces work");
    session
        .interrupt_transport_and_resume()
        .expect("resume after interruption");
    assert!(session.transport.interrupted);
    assert!(session.transport.resumed);
    assert!(session.metrics.resume_transfer_count >= 1);
}

#[test]
fn test_cross_machine_partition_detection() {
    let (_a_root, _b_root, mut session) = make_session();
    session
        .detect_partition()
        .expect("partition divergence is detected");
    assert!(session.partition_detected);
    assert!(session
        .machine_a
        .runtime
        .logs
        .iter()
        .any(|log| log.detected && log.rejected));
}

#[test]
fn test_cross_machine_long_duration_runtime() {
    let (_a_root, _b_root, mut session) = make_session();
    assert!(session.run_convergence(10_000).unwrap().converged());

    let (_a_root, _b_root, mut long_session) = make_session();
    assert!(long_session.run_convergence(50_000).unwrap().converged());
    assert_eq!(
        long_session.machine_a.runtime.replay_root(),
        long_session.machine_b.runtime.replay_root()
    );
}

#[test]
fn test_cross_machine_authority_preservation() {
    let (_a_root, _b_root, mut session) = make_session();
    assert!(session.authority_preserved());
}

#[test]
fn test_arena_vanguard_cross_machine_runtime() {
    let (_a_root, _b_root, mut session) = make_session();
    assert_eq!(session.machine_a.runtime.session.world_id, "arena-vanguard");
    assert!(session.run_convergence(1_000).unwrap().converged());
    session.certify_transport().unwrap();
    let checkpoint = session.transfer_checkpoint().unwrap();
    assert_eq!(checkpoint.source_machine, "machine-a");
    assert!(session.metrics.checkpoint_transfer_count >= 1);
    assert!(matches!(
        session.transport.mode,
        CrossMachineTransportMode::Tcp
    ));
}

#[test]
fn test_cross_machine_transport_modes() {
    let loopback = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 99);
    assert!(CrossMachineTransport::localhost_disabled()
        .validate_address(loopback)
        .is_err());
    assert!(CrossMachineTransport::machine_address(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(192, 0, 2, 10)),
        7777,
    ))
    .validate_address(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(192, 0, 2, 10)),
        7777
    ))
    .is_ok());
}
