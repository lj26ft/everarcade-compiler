use super::{bundle::ContinuityBundle, divergence::detect_divergence};

pub fn replay_peer_continuity(local: &ContinuityBundle, peer: &ContinuityBundle) -> bool {
    local.receipt_hashes == peer.receipt_hashes && local.execution_hashes == peer.execution_hashes
}

pub fn advance_federation_state(peer: &ContinuityBundle) -> ContinuityBundle {
    peer.clone()
}

pub fn reconcile_peer(
    local: &ContinuityBundle,
    peer: &ContinuityBundle,
) -> Option<ContinuityBundle> {
    if detect_divergence(local, peer).is_none() {
        Some(advance_federation_state(peer))
    } else {
        None
    }
}
