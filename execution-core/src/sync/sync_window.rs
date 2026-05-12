use crate::sync::replay_window::{validate_replay_window, ReplayWindow};

pub fn validate_sync_window(window: &ReplayWindow, prior_replay_root_matches: bool, final_replay_root_matches: bool) -> bool {
    prior_replay_root_matches && final_replay_root_matches && validate_replay_window(window)
}
