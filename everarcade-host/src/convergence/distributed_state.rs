#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DistributedState { pub replay_root: [u8; 32], pub checkpoint_root: [u8; 32] }
