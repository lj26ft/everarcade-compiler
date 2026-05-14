pub fn budget_pressure(allocated: u64, spent: u64) -> u64 {
    spent.saturating_sub(allocated)
}
