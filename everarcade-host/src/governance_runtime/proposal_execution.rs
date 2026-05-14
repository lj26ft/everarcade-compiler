pub type Hash = [u8; 32];
pub fn execute_proposal(proposal_root: Hash, scope_root: Hash) -> Hash {
    std::array::from_fn(|i| proposal_root[i] ^ scope_root[31 - i])
}
