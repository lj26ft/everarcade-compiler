pub type Hash = [u8; 32];
pub fn derive_window_checkpoint_root(window_replay_root: Hash, checkpoint_salt: u8) -> Hash { std::array::from_fn(|i| window_replay_root[i] ^ checkpoint_salt) }
