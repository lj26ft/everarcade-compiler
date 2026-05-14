pub fn checkpoint_cost(retained_checkpoints: u64, per_checkpoint_cost: u64) -> u64 {
    retained_checkpoints.saturating_mul(per_checkpoint_cost)
}
