use super::peer_sync_manifest::PeerSyncManifest;

pub fn has_recovery_windows(manifest: &PeerSyncManifest) -> bool {
    manifest.available_windows > 0
}
