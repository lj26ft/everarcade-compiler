#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionCheckpoint {
    pub session_id: String,
    pub checkpoint_index: u64,
}
