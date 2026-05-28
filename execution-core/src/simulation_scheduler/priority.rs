pub fn priority_score(partition_id: &str, tick: u64) -> u64 {
    partition_id.bytes().map(u64::from).sum::<u64>() + tick
}
