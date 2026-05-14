use super::{
    execution_assignment::ExecutionAssignment, execution_window::DistributedExecutionWindow,
};

pub fn assignment_matches_window(
    window: &DistributedExecutionWindow,
    assignment: &ExecutionAssignment,
) -> bool {
    window.window_id == assignment.window_id
        && window.assigned_operator == assignment.assigned_operator
}
