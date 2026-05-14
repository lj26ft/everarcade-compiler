pub type Hash = [u8; 32];
pub fn derive_execution_window_root(proposal_execution_root: Hash, quorum_root: Hash) -> Hash {
    std::array::from_fn(|i| proposal_execution_root[i] ^ quorum_root[i])
}
