pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrphanExecution {
    pub assignment_id: Hash,
    pub orphaned_operator: Hash,
}
