pub fn plan_action(entity_id: &str, tick: u64, memory_count: usize) -> String {
    format!(
        "{}:{}:{}",
        entity_id,
        tick,
        if memory_count % 2 == 0 {
            "observe"
        } else {
            "act"
        }
    )
}
