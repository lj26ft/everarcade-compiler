use super::replay_window::ReplayWindow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayExchangeRequest {
    pub window: ReplayWindow,
}
