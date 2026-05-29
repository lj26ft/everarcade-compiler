pub fn scheduler_order(tasks: &[&str]) -> String { let mut parts = vec!["scheduler-order"]; parts.extend_from_slice(tasks); crate::stable_hash(&parts) }
