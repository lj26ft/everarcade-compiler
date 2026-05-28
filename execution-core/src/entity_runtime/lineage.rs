pub fn lineage_root(entity_id: &str, generation: u64, identity_root: &str) -> String {
    format!("entity:{entity_id}:lineage:{generation}:{identity_root}")
}
