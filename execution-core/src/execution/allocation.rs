pub fn allocate_units(total_units: u64, active_queues: u64) -> u64 {
    if active_queues == 0 {
        0
    } else {
        total_units / active_queues
    }
}
