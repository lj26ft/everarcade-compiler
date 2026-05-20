use std::collections::BTreeMap;

use super::peer_session::{maintain_peer_session, recover_peer_session, PeerSession};

#[derive(Clone, Debug, Default)]
pub struct SessionCoordinator {
    pub active_sessions: BTreeMap<String, PeerSession>,
    pub sync_progress: BTreeMap<String, u64>,
    pub heartbeat_continuity: BTreeMap<String, bool>,
    pub recovery_state: BTreeMap<String, bool>,
    pub checkpoint_advancement: BTreeMap<String, u64>,
}

pub fn coordinate_sessions(coordinator: &mut SessionCoordinator, sessions: Vec<PeerSession>) {
    for mut session in sessions {
        maintain_peer_session(&mut session);
        coordinator
            .sync_progress
            .insert(session.peer.clone(), session.continuity_checkpoint);
        coordinator
            .heartbeat_continuity
            .insert(session.peer.clone(), session.connected && session.active);
        coordinator
            .recovery_state
            .insert(session.peer.clone(), false);
        coordinator
            .checkpoint_advancement
            .insert(session.peer.clone(), session.continuity_checkpoint);
        coordinator
            .active_sessions
            .insert(session.peer.clone(), session);
    }
}

pub fn monitor_session_health(coordinator: &SessionCoordinator, peer: &str) -> bool {
    coordinator
        .active_sessions
        .get(peer)
        .map(|s| s.active && s.connected && s.handshaken)
        .unwrap_or(false)
}

pub fn advance_runtime_sync(
    coordinator: &mut SessionCoordinator,
    peer: &str,
    checkpoint: u64,
) -> bool {
    let Some(session) = coordinator.active_sessions.get_mut(peer) else {
        return false;
    };
    if checkpoint <= session.continuity_checkpoint {
        return false;
    }
    session.continuity_checkpoint = checkpoint;
    session.synchronized = true;
    coordinator
        .sync_progress
        .insert(peer.to_string(), checkpoint);
    coordinator
        .checkpoint_advancement
        .insert(peer.to_string(), checkpoint);
    true
}

pub fn restore_runtime_sessions(coordinator: &mut SessionCoordinator) {
    for (peer, session) in coordinator.active_sessions.iter_mut() {
        recover_peer_session(session, session.continuity_checkpoint);
        coordinator.recovery_state.insert(peer.clone(), true);
    }
}

pub fn resume_checkpoint_advancement(
    coordinator: &mut SessionCoordinator,
    peer: &str,
    next_checkpoint: u64,
) -> bool {
    advance_runtime_sync(coordinator, peer, next_checkpoint)
}

pub fn recover_distributed_runtime(coordinator: &mut SessionCoordinator) -> bool {
    restore_runtime_sessions(coordinator);
    coordinator
        .active_sessions
        .values()
        .all(|s| s.active && s.connected)
}

pub fn recover_live_session(coordinator: &mut SessionCoordinator, peer: &str) -> bool {
    let Some(session) = coordinator.active_sessions.get_mut(peer) else {
        return false;
    };
    recover_peer_session(session, session.continuity_checkpoint);
    coordinator.recovery_state.insert(peer.to_string(), true);
    true
}

pub fn resume_partial_advancement(
    coordinator: &mut SessionCoordinator,
    peer: &str,
    checkpoint: u64,
) -> bool {
    resume_checkpoint_advancement(coordinator, peer, checkpoint)
}

pub fn restore_checkpoint_lineage(parent: [u8; 32], candidate: [u8; 32]) -> bool {
    parent == candidate
}

pub fn verify_runtime_recovery_state(coordinator: &SessionCoordinator, peer: &str) -> bool {
    coordinator
        .recovery_state
        .get(peer)
        .copied()
        .unwrap_or(false)
}

pub fn reject_invalid_live_continuity(
    out_of_order: bool,
    duplicate: bool,
    stale_checkpoint: bool,
) -> bool {
    out_of_order || duplicate || stale_checkpoint
}

pub fn verify_live_replay(journal_ok: bool, receipt_ok: bool, execution_ok: bool) -> bool {
    journal_ok && receipt_ok && execution_ok
}

pub fn verify_continuous_convergence(checkpoint_ok: bool, state_root_ok: bool) -> bool {
    checkpoint_ok && state_root_ok
}

pub fn detect_live_divergence(converged: bool) -> bool {
    !converged
}

pub fn inspect_live_continuity(coordinator: &SessionCoordinator) -> usize {
    coordinator.active_sessions.len()
}

pub fn inspect_runtime_advancement(coordinator: &SessionCoordinator, peer: &str) -> Option<u64> {
    coordinator.checkpoint_advancement.get(peer).copied()
}

pub fn inspect_recovery_state(coordinator: &SessionCoordinator, peer: &str) -> Option<bool> {
    coordinator.recovery_state.get(peer).copied()
}
