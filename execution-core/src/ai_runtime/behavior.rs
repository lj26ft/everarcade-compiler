pub fn deterministic_behavior(memory_count: usize) -> String {
    if memory_count % 2 == 0 {
        "observe"
    } else {
        "act"
    }
    .to_string()
}
