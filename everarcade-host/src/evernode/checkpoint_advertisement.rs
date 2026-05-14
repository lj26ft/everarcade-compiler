pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckpointAdvertisement {
    pub checkpoint_root: Hash,
    pub replay_root: Hash,
}
