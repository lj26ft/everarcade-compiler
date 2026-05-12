pub fn epoch_pressure(consumed: u64, budget: u64) -> u64 { consumed.saturating_sub(budget) }
