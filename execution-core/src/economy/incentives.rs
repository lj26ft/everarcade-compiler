pub fn incentive_balance(execution_rewards: u64, proof_rewards: u64, archival_rewards: u64, penalties: u64) -> i64 {
    execution_rewards.saturating_add(proof_rewards).saturating_add(archival_rewards) as i64 - penalties as i64
}
