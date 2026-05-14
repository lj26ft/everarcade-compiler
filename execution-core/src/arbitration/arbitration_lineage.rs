pub fn arbitration_lineage(parent: [u8; 32], child_parent: Option<[u8; 32]>) -> bool {
    child_parent == Some(parent)
}
