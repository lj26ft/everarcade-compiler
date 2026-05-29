pub fn node_status(node_id: &str, deployment_hash: &str) -> String { crate::stable_hash(&["node-status", node_id, deployment_hash]) }
