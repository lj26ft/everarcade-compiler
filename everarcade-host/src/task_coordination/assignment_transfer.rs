use super::task_assignment::{Hash, TaskAssignment};

pub fn transfer_assignment(assignment: &TaskAssignment, next_operator: Hash) -> TaskAssignment {
    TaskAssignment {
        assignment_id: assignment.assignment_id,
        task_root: assignment.task_root,
        assigned_operator: next_operator,
        parent_assignment: Some(assignment.assignment_id),
    }
}
