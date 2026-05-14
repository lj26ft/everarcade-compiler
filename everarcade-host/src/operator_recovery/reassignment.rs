use crate::task_coordination::{
    assignment_transfer::transfer_assignment,
    task_assignment::{Hash, TaskAssignment},
};

pub fn reassign_on_failure(
    assignment: &TaskAssignment,
    replacement_operator: Hash,
) -> TaskAssignment {
    transfer_assignment(assignment, replacement_operator)
}
