pub fn cache_key(replay_tip: &str) -> String {
    format!("observer-world:cache:{replay_tip}")
}
