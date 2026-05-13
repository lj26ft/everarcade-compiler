use super::queue_state::QueueState;
#[derive(Debug, Clone)]
pub struct QueueCheckpoint {
    pub id: String,
    pub state: QueueState,
}
