pub fn fairness_index(total_allocated: u64, participants: u64) -> u64 {
    if participants == 0 {
        0
    } else {
        total_allocated / participants
    }
}
