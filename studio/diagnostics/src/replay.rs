pub fn replay_continuity(replay_root: &str, partition_root: &str, civilization_root: &str) -> String { crate::stable_hash(&["diagnostic-replay", replay_root, partition_root, civilization_root]) }
