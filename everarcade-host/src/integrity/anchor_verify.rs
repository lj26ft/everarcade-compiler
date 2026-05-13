pub fn verify_anchor_root(actual: [u8; 32], expected: [u8; 32]) -> bool {
    actual == expected
}
