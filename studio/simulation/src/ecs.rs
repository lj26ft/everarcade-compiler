pub fn ecs_execution_order(systems: &[&str]) -> String { let mut parts = vec!["ecs-order"]; parts.extend_from_slice(systems); crate::stable_hash(&parts) }
