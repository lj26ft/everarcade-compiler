use crate::distributed_execution::workload_partition::{Hash, WorkloadPartition};
use crate::task_coordination::partition_transfer::transfer_partition;

pub fn failover_partition(partition: WorkloadPartition, replacement: Hash) -> WorkloadPartition {
    transfer_partition(partition, replacement)
}
