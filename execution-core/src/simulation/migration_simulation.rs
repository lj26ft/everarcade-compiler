pub fn migration_continuity_valid(
    source: [u8; 32],
    target: [u8; 32],
    continuity_root: [u8; 32],
) -> bool {
    source != target && continuity_root != [0; 32]
}
