pub fn federation_lineage(nodes: &[&str], deployment_hash: &str) -> String { let mut parts = vec!["federation-lineage", deployment_hash]; parts.extend_from_slice(nodes); crate::stable_hash(&parts) }
