pub fn validate_epoch_continuity(indices: &[u64]) -> bool {
    indices.windows(2).all(|w| w[1] == w[0] + 1)
}
