pub fn highest_valid_replay(local_height: u64, remote_height: u64, remote_valid: bool) -> u64 {
    if remote_valid && remote_height > local_height {
        remote_height
    } else {
        local_height
    }
}
