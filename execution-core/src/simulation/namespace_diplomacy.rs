pub fn namespace_conflict_resolved(resolution_root: [u8; 32]) -> bool {
    resolution_root != [0; 32]
}
