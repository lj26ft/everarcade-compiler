pub fn settlement_anchor_cost(anchors: u64, proofs: u64, checkpoints: u64) -> u64 {
    anchors
        .saturating_mul(2)
        .saturating_add(proofs.saturating_mul(5))
        .saturating_add(checkpoints.saturating_mul(7))
}
