pub fn has_replay_divergence(local: [u8; 32], remote: [u8; 32]) -> bool {
    local != remote
}
