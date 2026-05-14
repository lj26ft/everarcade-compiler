pub fn validate_ordered_and_contiguous(heights: &[u64]) -> bool {
    heights.windows(2).all(|w| w[1] == w[0] + 1)
}
