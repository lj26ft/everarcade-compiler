pub fn slash_amount(stake: u64, violations: u64) -> u64 {
    let pct = (violations.saturating_mul(10)).min(100);
    stake.saturating_mul(pct) / 100
}
