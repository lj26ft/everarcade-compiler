pub fn verifier_reward(replays: u64, challenges: u64, archival_checks: u64) -> u64 {
    replays.saturating_mul(3)
        .saturating_add(challenges.saturating_mul(9))
        .saturating_add(archival_checks.saturating_mul(4))
}
