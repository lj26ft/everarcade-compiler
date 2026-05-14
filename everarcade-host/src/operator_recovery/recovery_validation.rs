use crate::task_coordination::task_assignment::TaskAssignment;

pub fn validate_recovery(previous: &TaskAssignment, recovered: &TaskAssignment) -> bool {
    previous.task_root == recovered.task_root
}
