pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionCommit {
    pub window_id: Hash,
    pub assignment_id: Hash,
    pub execution_root: Hash,
    pub receipt_root: Hash,
}
