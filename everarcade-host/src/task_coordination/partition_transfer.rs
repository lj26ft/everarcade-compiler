use crate::distributed_execution::workload_partition::{Hash, WorkloadPartition};

pub fn transfer_partition(
    mut partition: WorkloadPartition,
    new_operator: Hash,
) -> WorkloadPartition {
    partition.assigned_operator = new_operator;
    partition
}
