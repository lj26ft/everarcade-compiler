pub fn storage_growth(previous: u64, current: u64) -> u64 { current.saturating_sub(previous) }
