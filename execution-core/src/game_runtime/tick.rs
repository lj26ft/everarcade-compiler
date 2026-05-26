use crate::hashing::hash_bytes;

pub fn advance_tick(current: u64) -> u64 {
    current + 1
}

pub fn window_roots(tick: u64, state_root: &str, event_root: &str) -> (String, String, String) {
    let replay_root = hash_bytes(format!("{tick}:{state_root}:{event_root}").as_bytes());
    let validation_root = hash_bytes(replay_root.as_bytes());
    (
        state_root.to_string(),
        event_root.to_string(),
        validation_root,
    )
}
