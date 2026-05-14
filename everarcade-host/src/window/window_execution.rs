pub type Hash = [u8; 32];
pub fn derive_window_execution_root(window_id: u64, governance_root: Hash) -> Hash { std::array::from_fn(|i| governance_root[i] ^ ((window_id as u8).wrapping_add(i as u8))) }
