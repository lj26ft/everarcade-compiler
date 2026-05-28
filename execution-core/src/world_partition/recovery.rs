use super::runtime::WorldPartitionRuntime;
pub fn restore_partitions(runtime: &WorldPartitionRuntime) -> WorldPartitionRuntime {
    runtime.clone()
}
