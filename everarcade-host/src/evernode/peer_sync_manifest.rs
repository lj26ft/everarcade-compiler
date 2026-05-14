pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerSyncManifest {
    pub package_root: Hash,
    pub latest_checkpoint_root: Hash,
    pub replay_root: Hash,
    pub available_windows: u64,
}
