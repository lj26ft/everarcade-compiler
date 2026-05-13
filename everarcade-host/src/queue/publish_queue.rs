use super::queue_state::QueueState;
#[derive(Debug, Clone)]
pub struct PublishItem {
    pub id: String,
    pub state: QueueState,
}
