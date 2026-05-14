use super::task_assignment::TaskAssignment;

pub fn is_valid_assignment(assignment: &TaskAssignment) -> bool {
    assignment.task_root != [0u8; 32] && assignment.assigned_operator != [0u8; 32]
}
