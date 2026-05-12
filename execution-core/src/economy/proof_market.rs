pub fn proof_reward(proof_units: u64, recursion_depth: u64) -> u64 {
    proof_units.saturating_mul(5).saturating_add(recursion_depth.saturating_mul(13))
}
