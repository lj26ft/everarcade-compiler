use super::runtime::WorldPartitionRuntime;
pub fn partition_runtime_is_deterministic(runtime: &WorldPartitionRuntime) -> bool {
    runtime
        .partitions
        .windows(2)
        .all(|w| w[0].partition_id <= w[1].partition_id)
        && runtime.replay_roots.iter().all(|r| !r.is_empty())
}
pub fn partition_runtime_equivalent(a: &WorldPartitionRuntime, b: &WorldPartitionRuntime) -> bool {
    a == b
}
