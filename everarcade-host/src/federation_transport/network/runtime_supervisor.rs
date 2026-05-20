use std::collections::BTreeMap;

use super::session_coordinator::{
    advance_runtime_sync, detect_live_divergence, inspect_runtime_advancement,
    recover_live_session, verify_continuous_convergence, verify_live_replay, SessionCoordinator,
};

#[derive(Clone, Debug, Default)]
pub struct RuntimeSupervisorState {
    pub health: BTreeMap<String, bool>,
    pub recovery_count: BTreeMap<String, u64>,
    pub replay_verified: BTreeMap<String, bool>,
    pub checkpoint_history: BTreeMap<String, Vec<u64>>,
    pub divergence_events: u64,
}

pub fn start_runtime_supervisor(coordinator: &SessionCoordinator) -> RuntimeSupervisorState {
    let mut state = RuntimeSupervisorState::default();
    for (peer, session) in &coordinator.active_sessions {
        state.health.insert(
            peer.clone(),
            session.active && session.connected && session.handshaken,
        );
        state.recovery_count.insert(peer.clone(), 0);
        state.replay_verified.insert(peer.clone(), true);
        state
            .checkpoint_history
            .entry(peer.clone())
            .or_default()
            .push(session.continuity_checkpoint);
    }
    state
}

pub fn monitor_runtime_health(
    coordinator: &SessionCoordinator,
    supervisor: &mut RuntimeSupervisorState,
) -> bool {
    for (peer, session) in &coordinator.active_sessions {
        let healthy = session.active && session.connected && session.handshaken;
        supervisor.health.insert(peer.clone(), healthy);
    }
    supervisor.health.values().all(|healthy| *healthy)
}

pub fn detect_sync_stall(last_checkpoint: u64, current_checkpoint: u64) -> bool {
    current_checkpoint <= last_checkpoint
}

pub fn detect_runtime_failure(
    coordinator: &SessionCoordinator,
    supervisor: &RuntimeSupervisorState,
    peer: &str,
) -> bool {
    let Some(session) = coordinator.active_sessions.get(peer) else {
        return true;
    };
    let healthy = supervisor.health.get(peer).copied().unwrap_or(false);
    let checkpoint = inspect_runtime_advancement(coordinator, peer).unwrap_or_default();
    !healthy || !session.connected || detect_sync_stall(checkpoint, session.continuity_checkpoint)
}

pub fn coordinate_runtime_recovery(
    coordinator: &mut SessionCoordinator,
    supervisor: &mut RuntimeSupervisorState,
    peer: &str,
) -> bool {
    if !recover_runtime_session(coordinator, supervisor, peer) {
        return false;
    }
    resume_autonomous_sync(coordinator, supervisor, peer)
}

pub fn recover_runtime_session(
    coordinator: &mut SessionCoordinator,
    supervisor: &mut RuntimeSupervisorState,
    peer: &str,
) -> bool {
    if !recover_live_session(coordinator, peer) {
        return false;
    }
    let count = supervisor
        .recovery_count
        .entry(peer.to_string())
        .or_insert(0);
    *count = count.saturating_add(1);
    true
}

pub fn recover_live_continuity(
    coordinator: &mut SessionCoordinator,
    supervisor: &mut RuntimeSupervisorState,
    peer: &str,
    checkpoint: u64,
) -> bool {
    let restored = advance_runtime_sync(coordinator, peer, checkpoint);
    if restored {
        supervisor
            .checkpoint_history
            .entry(peer.to_string())
            .or_default()
            .push(checkpoint);
    }
    restored
}

pub fn resume_autonomous_sync(
    coordinator: &mut SessionCoordinator,
    supervisor: &mut RuntimeSupervisorState,
    peer: &str,
) -> bool {
    let Some(session) = coordinator.active_sessions.get(peer) else {
        return false;
    };
    supervisor
        .health
        .insert(peer.to_string(), session.active && session.connected);
    true
}

pub fn advance_live_continuity(
    coordinator: &mut SessionCoordinator,
    supervisor: &mut RuntimeSupervisorState,
    peer: &str,
    next_checkpoint: u64,
) -> bool {
    if !advance_runtime_sync(coordinator, peer, next_checkpoint) {
        return false;
    }
    supervisor
        .checkpoint_history
        .entry(peer.to_string())
        .or_default()
        .push(next_checkpoint);
    true
}

pub fn coordinate_incremental_advancement(
    coordinator: &mut SessionCoordinator,
    supervisor: &mut RuntimeSupervisorState,
    peer: &str,
    next_checkpoint: u64,
) -> bool {
    if reject_invalid_runtime_progression(
        inspect_runtime_advancement(coordinator, peer).unwrap_or(0) >= next_checkpoint,
        false,
        false,
        false,
        false,
    ) {
        return false;
    }
    advance_live_continuity(coordinator, supervisor, peer, next_checkpoint)
}

pub fn verify_live_advancement(prev: u64, next: u64) -> bool {
    next > prev
}

pub fn verify_continuous_integrity(
    receipt_ok: bool,
    journal_ok: bool,
    checkpoint_ok: bool,
    execution_ok: bool,
) -> bool {
    receipt_ok && journal_ok && checkpoint_ok && execution_ok
}

pub fn verify_live_state_root_convergence(local: [u8; 32], remote: [u8; 32]) -> bool {
    local == remote
}

pub fn verify_live_replay_continuity(
    journal_ok: bool,
    receipt_ok: bool,
    execution_ok: bool,
) -> bool {
    verify_live_replay(journal_ok, receipt_ok, execution_ok)
}

pub fn validate_recovery_history(non_decreasing: bool, uncorrupted: bool) -> bool {
    non_decreasing && uncorrupted
}

pub fn verify_supervisor_state(state: &RuntimeSupervisorState) -> bool {
    !state.health.is_empty() && state.recovery_count.len() == state.health.len()
}

pub fn reject_invalid_runtime_progression(
    stale_checkpoint: bool,
    invalid_replay: bool,
    corrupted_history: bool,
    duplicate_replay: bool,
    out_of_order_sync: bool,
) -> bool {
    stale_checkpoint || invalid_replay || corrupted_history || duplicate_replay || out_of_order_sync
}

pub fn inspect_runtime_health(supervisor: &RuntimeSupervisorState, peer: &str) -> Option<bool> {
    supervisor.health.get(peer).copied()
}

pub fn inspect_supervisor_state(supervisor: &RuntimeSupervisorState) -> usize {
    supervisor.health.len()
}

pub fn inspect_live_convergence(checkpoint_ok: bool, state_root_ok: bool, replay_ok: bool) -> bool {
    let converged = verify_continuous_convergence(checkpoint_ok, state_root_ok) && replay_ok;
    detect_live_divergence(converged)
}

pub fn detect_live_divergence_event(
    supervisor: &mut RuntimeSupervisorState,
    converged: bool,
) -> bool {
    let diverged = detect_live_divergence(converged);
    if diverged {
        supervisor.divergence_events = supervisor.divergence_events.saturating_add(1);
    }
    diverged
}
