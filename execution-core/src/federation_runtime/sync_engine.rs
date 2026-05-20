use super::{
    bundle::ContinuityBundle,
    error::FederationRuntimeError,
    replay_verification::verify_peer_replay,
    topology_state::{PeerContinuityState, PeerStatus, TopologyStateEngine},
};

pub fn sync_peer(
    topology: &mut TopologyStateEngine,
    peer_id: [u8; 32],
    local: &ContinuityBundle,
    peer: &ContinuityBundle,
) -> Result<(), FederationRuntimeError> {
    synchronize_continuity(local, peer)?;
    advance_peer_checkpoint(topology, peer_id, peer.checkpoint_hash)
}

pub fn synchronize_continuity(
    local: &ContinuityBundle,
    peer: &ContinuityBundle,
) -> Result<(), FederationRuntimeError> {
    verify_peer_replay(local, peer)
}

pub fn advance_peer_checkpoint(
    topology: &mut TopologyStateEngine,
    peer_id: [u8; 32],
    checkpoint_hash: [u8; 32],
) -> Result<(), FederationRuntimeError> {
    let mut peer_state = topology
        .peer_state(peer_id)
        .cloned()
        .unwrap_or(PeerContinuityState::new(peer_id));
    peer_state.last_checkpoint = checkpoint_hash;
    peer_state.status = PeerStatus::Active;
    topology.upsert_peer(peer_state);
    Ok(())
}

pub fn request_journal_range(start: u64, end: u64) -> Result<Vec<u64>, FederationRuntimeError> {
    if end < start {
        return Err(FederationRuntimeError::InvalidTopologyState);
    }
    Ok((start..=end).collect())
}

pub fn verify_journal_range(entries: &[u64]) -> Result<(), FederationRuntimeError> {
    for w in entries.windows(2) {
        if w[1] != w[0] + 1 {
            return Err(FederationRuntimeError::ContinuityVerificationFailed);
        }
    }
    Ok(())
}

pub fn apply_journal_range(
    local_tail: u64,
    entries: &[u64],
) -> Result<u64, FederationRuntimeError> {
    verify_journal_range(entries)?;
    if let Some(first) = entries.first() {
        if *first != local_tail + 1 {
            return Err(FederationRuntimeError::ContinuityVerificationFailed);
        }
    }
    Ok(entries.last().copied().unwrap_or(local_tail))
}
