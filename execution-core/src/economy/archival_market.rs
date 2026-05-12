pub fn archival_reward(byte_months: u64) -> u64 {
    (byte_months / 1024).saturating_add(1)
}
