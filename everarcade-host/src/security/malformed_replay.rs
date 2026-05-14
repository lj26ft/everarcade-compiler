pub type Hash = [u8; 32];
pub fn malformed_replay_detected(replay_root: Hash) -> bool { replay_root == [0; 32] }
