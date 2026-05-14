use super::root_comparison::roots_match;

pub fn reject_on_divergence(local_replay_root: [u8; 32], remote_replay_root: [u8; 32]) -> bool {
    !roots_match(local_replay_root, remote_replay_root)
}
