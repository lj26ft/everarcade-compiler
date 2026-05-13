use super::queue_state::QueueState;
pub fn is_terminal(state: QueueState) -> bool {
    matches!(state, QueueState::Complete | QueueState::Failed)
}
