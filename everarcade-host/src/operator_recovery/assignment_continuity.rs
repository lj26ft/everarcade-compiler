use crate::distributed_execution::workload_partition::WorkloadPartition;

pub fn continuity_preserved(before: &WorkloadPartition, after: &WorkloadPartition) -> bool {
    before.partition_root == after.partition_root && before.package_root == after.package_root
}
