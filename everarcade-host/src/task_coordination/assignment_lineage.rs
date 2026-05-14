use super::task_assignment::{Hash, TaskAssignment};

pub fn lineage_chain_tip(assignments: &[TaskAssignment]) -> Option<Hash> {
    assignments.last().map(|a| a.assignment_id)
}
