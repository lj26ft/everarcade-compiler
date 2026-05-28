pub fn validate_migration_state(source: &str, target: &str) -> bool { !source.is_empty() && !target.is_empty() && source != target }
