pub fn dedupe_transfer_windows(windows: &[u64]) -> Vec<u64> {
    let mut v = windows.to_vec();
    v.sort_unstable();
    v.dedup();
    v
}
