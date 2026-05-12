pub fn minimum_stake(reputation_score: u64) -> u64 {
    1_000u64.saturating_sub(reputation_score.min(900))
}
