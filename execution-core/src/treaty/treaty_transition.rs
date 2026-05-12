use super::treaty::{ExecutionTreaty, Hash};

pub fn amend_treaty(treaty: &ExecutionTreaty, next_capability_scope_root: Hash) -> ExecutionTreaty {
    let mut next = treaty.clone();
    next.capability_scope_root = next_capability_scope_root;
    next
}
