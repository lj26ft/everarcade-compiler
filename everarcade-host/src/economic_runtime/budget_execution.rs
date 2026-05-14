pub type Hash = [u8; 32];
pub fn execute_budget_window(budget_root: Hash, treaty_root: Hash) -> Hash {
    std::array::from_fn(|i| budget_root[i] ^ treaty_root[i])
}
