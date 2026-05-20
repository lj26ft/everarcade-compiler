use super::{
    bundle::ContinuityBundle, divergence::detect_divergence, error::FederationRuntimeError,
    replay_verification::verify_peer_replay,
};

pub fn reconcile_peer_state(
    local: &ContinuityBundle,
    peer: &ContinuityBundle,
) -> Result<ContinuityBundle, FederationRuntimeError> {
    if detect_divergence(local, peer).is_some() {
        return Err(FederationRuntimeError::ContinuityVerificationFailed);
    }
    verify_peer_replay(local, peer)?;
    Ok(peer.clone())
}

pub fn reconcile_checkpoint_chain(
    local: &ContinuityBundle,
    peer: &ContinuityBundle,
) -> Result<[u8; 32], FederationRuntimeError> {
    if local.checkpoint_hash != peer.checkpoint_hash {
        return Err(FederationRuntimeError::ContinuityVerificationFailed);
    }
    Ok(peer.checkpoint_hash)
}

pub fn reconcile_journal_chain(
    local: &ContinuityBundle,
    peer: &ContinuityBundle,
) -> Result<[u8; 32], FederationRuntimeError> {
    if local.journal_hash != peer.journal_hash {
        return Err(FederationRuntimeError::ContinuityVerificationFailed);
    }
    Ok(peer.journal_hash)
}
