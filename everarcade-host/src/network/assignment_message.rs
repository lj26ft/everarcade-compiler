pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssignmentMessage {
    pub assignment_id: Hash,
    pub assigned_operator: Hash,
}
