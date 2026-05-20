use super::{bundle::ContinuityBundle, divergence::compare_continuity_roots};

pub fn sync_checkpoint(local: &ContinuityBundle, peer: &ContinuityBundle) -> bool {
    compare_continuity_roots(local.continuity_hash, peer.continuity_hash)
}

pub fn sync_journal_range(start: u64, end: u64) -> Vec<u64> {
    (start..=end).collect()
}

pub fn request_continuity_bundle(bundle: &ContinuityBundle) -> ContinuityBundle {
    bundle.clone()
}

pub fn verify_peer_checkpoint(local: &ContinuityBundle, peer: &ContinuityBundle) -> bool {
    local.checkpoint_hash == peer.checkpoint_hash && local.checkpoint_hash == peer.checkpoint_hash
}
