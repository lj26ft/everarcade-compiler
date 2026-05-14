pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskAssignment {
    pub assignment_id: Hash,
    pub task_root: Hash,
    pub assigned_operator: Hash,
    pub parent_assignment: Option<Hash>,
}
