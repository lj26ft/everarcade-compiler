#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckpointResponse {
    pub accepted: bool,
    pub count: u64,
}
