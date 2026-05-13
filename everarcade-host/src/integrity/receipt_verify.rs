pub fn verify_receipt_root(actual: [u8; 32], expected: [u8; 32]) -> bool {
    actual == expected
}
