use crate::task_coordination::task_assignment::TaskAssignment;

pub fn reconcile_assignments(
    local: &[TaskAssignment],
    remote: &[TaskAssignment],
) -> Vec<TaskAssignment> {
    if remote.len() > local.len() {
        remote.to_vec()
    } else {
        local.to_vec()
    }
}
