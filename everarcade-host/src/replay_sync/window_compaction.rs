use super::replay_window::ReplayWindow;

pub fn compact_windows(windows: Vec<ReplayWindow>) -> Vec<ReplayWindow> {
    windows
        .into_iter()
        .filter(|w| w.receipt_count > 0)
        .collect()
}
