pub fn world_structure(world_id: &str, partitions: &[&str]) -> String { let mut parts = vec!["world-structure", world_id]; parts.extend_from_slice(partitions); crate::stable_hash(&parts) }
