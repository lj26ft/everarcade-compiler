pub fn operational_health(statuses: &[&str]) -> String { let mut parts = vec!["operational-health"]; parts.extend_from_slice(statuses); crate::stable_hash(&parts) }
