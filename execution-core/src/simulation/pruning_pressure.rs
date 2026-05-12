pub fn pruning_pressure(used_storage: u64, storage_budget: u64) -> u64 { used_storage.saturating_sub(storage_budget) }
