use super::replay_window::ReplayWindow;

pub fn validate_window(window: &ReplayWindow) -> bool {
    window.receipt_count > 0
}
