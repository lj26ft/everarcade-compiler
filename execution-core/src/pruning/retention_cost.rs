pub fn retention_cost(retained_items: u64, unit_cost: u64) -> u64 { retained_items.saturating_mul(unit_cost) }
