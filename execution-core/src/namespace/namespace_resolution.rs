pub fn resolve_namespace_conflict(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    if left <= right { left } else { right }
}
