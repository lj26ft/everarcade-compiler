pub fn fairness_score(total_slots: u64, consumed_slots: u64) -> u64 {
    total_slots.saturating_sub(consumed_slots)
}
