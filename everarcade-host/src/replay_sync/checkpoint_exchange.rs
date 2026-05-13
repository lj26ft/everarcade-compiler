pub fn validate_checkpoint_lineage(parent: [u8; 32], expected_parent: [u8; 32]) -> bool {
    parent == expected_parent
}
