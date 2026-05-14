pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionAssignment {
    pub window_id: Hash,
    pub task_root: Hash,
    pub assigned_operator: Hash,
}
