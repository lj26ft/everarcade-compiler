use everarcade_host::evernode::{
    peer_sync_manifest::PeerSyncManifest, recovery_discovery::has_recovery_windows,
};

#[test]
fn evernode_manifest_advertises_windows() {
    let manifest = PeerSyncManifest {
        package_root: [1; 32],
        latest_checkpoint_root: [2; 32],
        replay_root: [3; 32],
        available_windows: 4,
    };
    assert!(has_recovery_windows(&manifest));
}
