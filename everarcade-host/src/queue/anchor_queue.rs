use super::queue_state::QueueState;
#[derive(Debug, Clone)]
pub struct AnchorItem {
    pub id: String,
    pub state: QueueState,
}
