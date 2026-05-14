pub type Hash = [u8; 32];
pub fn derive_window_replay_root(prior_replay_root: Hash, execution_root: Hash) -> Hash {
    std::array::from_fn(|i| prior_replay_root[i].wrapping_add(execution_root[i]))
}
