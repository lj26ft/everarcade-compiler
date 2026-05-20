use execution_core::federation_runtime::bundle::ContinuityBundle;

pub fn sync_checkpoint(local: &ContinuityBundle, peer: &ContinuityBundle) -> bool {
    local.checkpoint_hash == peer.checkpoint_hash
}

pub fn sync_journal_range(start: u64, end: u64) -> Vec<u64> {
    (start..=end).collect()
}

pub fn request_continuity_bundle(peer: &ContinuityBundle) -> ContinuityBundle {
    peer.clone()
}
