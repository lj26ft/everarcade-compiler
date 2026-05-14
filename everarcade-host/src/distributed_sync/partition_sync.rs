use crate::distributed_execution::workload_partition::WorkloadPartition;

pub fn sync_partitions(partitions: &[WorkloadPartition]) -> Vec<WorkloadPartition> {
    partitions.to_vec()
}
