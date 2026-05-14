pub type Hash = [u8; 32];
pub fn derive_treasury_window_root(previous_treasury_root: Hash, budget_execution_root: Hash) -> Hash { std::array::from_fn(|i| previous_treasury_root[i].wrapping_add(budget_execution_root[i])) }
