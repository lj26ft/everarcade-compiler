use sha2::{Digest, Sha256};

use super::task_assignment::{Hash, TaskAssignment};

pub fn deterministic_assignment(
    task_root: Hash,
    operators: &[Hash],
    parent_assignment: Option<Hash>,
) -> TaskAssignment {
    let mut hasher = Sha256::new();
    hasher.update(task_root);
    if let Some(parent) = parent_assignment {
        hasher.update(parent);
    }
    let assignment_id: Hash = hasher.finalize().into();
    let idx = if operators.is_empty() {
        0
    } else {
        assignment_id[0] as usize % operators.len()
    };
    let assigned_operator = operators.get(idx).copied().unwrap_or([0u8; 32]);
    TaskAssignment {
        assignment_id,
        task_root,
        assigned_operator,
        parent_assignment,
    }
}
