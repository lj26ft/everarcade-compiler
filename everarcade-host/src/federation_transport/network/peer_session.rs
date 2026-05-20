use std::collections::BTreeSet;
use std::time::Instant;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerSession {
    pub peer: String,
    pub topology_epoch: u64,
    pub lease_id: String,
    pub active: bool,
    pub connected: bool,
    pub handshaken: bool,
    pub synchronized: bool,
    pub continuity_checkpoint: u64,
    pub last_execution_hash: [u8; 32],
    pub last_state_root: [u8; 32],
    pub last_journal_hash: [u8; 32],
    pub last_seen: Option<Instant>,
    pub missed_ranges: BTreeSet<(u64, u64)>,
}

pub fn establish_peer_session(peer: String, topology_epoch: u64, lease_id: String) -> PeerSession {
    PeerSession {
        peer,
        topology_epoch,
        lease_id,
        active: true,
        connected: true,
        handshaken: false,
        synchronized: false,
        continuity_checkpoint: 0,
        last_execution_hash: [0u8; 32],
        last_state_root: [0u8; 32],
        last_journal_hash: [0u8; 32],
        last_seen: Some(Instant::now()),
        missed_ranges: BTreeSet::new(),
    }
}

pub fn verify_peer_session(
    session: &PeerSession,
    expected_epoch: u64,
    expected_lease: &str,
) -> bool {
    session.active
        && session.connected
        && session.topology_epoch == expected_epoch
        && session.lease_id == expected_lease
}

pub fn start_peer_session(peer: String, topology_epoch: u64, lease_id: String) -> PeerSession {
    let mut session = establish_peer_session(peer, topology_epoch, lease_id);
    session.handshaken = true;
    session.synchronized = true;
    session
}

pub fn maintain_peer_session(session: &mut PeerSession) {
    session.active = true;
    session.connected = true;
    session.last_seen = Some(Instant::now());
}

pub fn recover_peer_session(session: &mut PeerSession, from_checkpoint: u64) {
    session.active = true;
    session.connected = true;
    session.handshaken = true;
    session.synchronized = false;
    session.continuity_checkpoint = from_checkpoint;
}

pub fn resume_peer_session(session: &mut PeerSession) {
    session.active = true;
    session.connected = true;
    session.handshaken = true;
    session.synchronized = true;
    session.last_seen = Some(Instant::now());
}

pub fn terminate_peer_session(session: &mut PeerSession) {
    session.active = false;
    session.connected = false;
    session.handshaken = false;
}

pub fn shutdown_peer_session(session: &mut PeerSession) {
    terminate_peer_session(session);
    session.synchronized = false;
}

pub fn advance_incremental_continuity(
    session: &mut PeerSession,
    previous_hash: [u8; 32],
    execution_hash: [u8; 32],
    state_root: [u8; 32],
    new_journal_hash: [u8; 32],
) -> bool {
    if session.last_journal_hash != [0u8; 32] && session.last_journal_hash != previous_hash {
        return false;
    }
    session.last_execution_hash = execution_hash;
    session.last_state_root = state_root;
    session.last_journal_hash = new_journal_hash;
    session.continuity_checkpoint = session.continuity_checkpoint.saturating_add(1);
    true
}

pub fn verify_incremental_checkpoint(session: &PeerSession, expected_checkpoint: u64) -> bool {
    session.continuity_checkpoint == expected_checkpoint
}

pub fn recover_interrupted_sync(session: &mut PeerSession, start: u64, end: u64) {
    session.missed_ranges.insert((start, end));
}

pub fn resume_missing_range_sync(session: &mut PeerSession, start: u64, end: u64) -> bool {
    session.missed_ranges.remove(&(start, end))
}

pub fn recover_checkpoint_continuity(session: &mut PeerSession, checkpoint: u64) {
    session.continuity_checkpoint = checkpoint;
}

pub fn verify_checkpoint_lineage(local_parent: [u8; 32], remote_parent: [u8; 32]) -> bool {
    local_parent == remote_parent
}

pub fn verify_execution_continuity(local_execution: [u8; 32], remote_execution: [u8; 32]) -> bool {
    local_execution == remote_execution
}

pub fn validate_continuity_advancement(
    lineage_ok: bool,
    receipt_ok: bool,
    execution_ok: bool,
    state_root_ok: bool,
    lease_ok: bool,
    topology_ok: bool,
) -> bool {
    lineage_ok && receipt_ok && execution_ok && state_root_ok && lease_ok && topology_ok
}

pub fn validate_recovery_state(has_partial_advancement: bool, has_invalid_state: bool) -> bool {
    !has_partial_advancement && !has_invalid_state
}

pub fn verify_incremental_replay(is_in_order: bool, proof_valid: bool) -> bool {
    is_in_order && proof_valid
}

pub fn reject_invalid_advancement(duplicate: bool, out_of_order: bool) -> bool {
    duplicate || out_of_order
}
