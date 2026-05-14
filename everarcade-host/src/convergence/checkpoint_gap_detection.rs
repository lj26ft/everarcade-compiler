pub fn detect_checkpoint_gap(local: u64, remote: u64) -> Option<u64> {
    (remote > local).then_some(remote - local)
}
