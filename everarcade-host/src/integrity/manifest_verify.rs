pub fn verify_manifest_root(actual: [u8; 32], expected: [u8; 32]) -> bool {
    actual == expected
}
