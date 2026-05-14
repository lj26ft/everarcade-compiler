pub fn reputation_score(successful_epochs: u64, violations: u64) -> u64 {
    successful_epochs
        .saturating_mul(10)
        .saturating_sub(violations.saturating_mul(25))
}
