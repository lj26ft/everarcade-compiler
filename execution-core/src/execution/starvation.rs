pub fn is_starving(unserved_rounds: u64, starvation_threshold: u64) -> bool {
    unserved_rounds >= starvation_threshold
}
