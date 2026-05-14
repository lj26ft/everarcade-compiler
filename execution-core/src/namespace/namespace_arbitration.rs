pub fn namespace_dispute_resolved(resolution_root: [u8; 32]) -> bool {
    resolution_root != [0; 32]
}
