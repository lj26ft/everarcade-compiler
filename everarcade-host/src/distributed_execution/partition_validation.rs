use super::workload_partition::WorkloadPartition;

pub fn validate_partition(partition: &WorkloadPartition) -> bool {
    partition.partition_id == partition.partition_root
        && partition.package_root != [0u8; 32]
        && partition.execution_window != [0u8; 32]
}
