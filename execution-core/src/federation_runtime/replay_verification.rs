use super::{bundle::ContinuityBundle, error::FederationRuntimeError};

pub fn verify_peer_replay(
    local: &ContinuityBundle,
    peer: &ContinuityBundle,
) -> Result<(), FederationRuntimeError> {
    verify_peer_state_root(local, peer)?;
    verify_peer_execution_hashes(local, peer)
}

pub fn verify_peer_state_root(
    local: &ContinuityBundle,
    peer: &ContinuityBundle,
) -> Result<(), FederationRuntimeError> {
    if local.state_root != peer.state_root {
        return Err(FederationRuntimeError::ContinuityVerificationFailed);
    }
    Ok(())
}

pub fn verify_peer_execution_hashes(
    local: &ContinuityBundle,
    peer: &ContinuityBundle,
) -> Result<(), FederationRuntimeError> {
    if local.execution_hashes != peer.execution_hashes
        || local.receipt_hashes != peer.receipt_hashes
    {
        return Err(FederationRuntimeError::ContinuityVerificationFailed);
    }
    Ok(())
}
