use super::partition_assignment::assign_partition;
use super::partition_root::derive_partition_root;
use super::workload_partition::{Hash, WorkloadPartition};

pub fn schedule_partitions(
    package_root: Hash,
    execution_window: Hash,
    input_roots: &[Hash],
    operators: &[Hash],
) -> Vec<WorkloadPartition> {
    input_roots
        .iter()
        .enumerate()
        .map(|(index, input_root)| {
            let partition_root = derive_partition_root(package_root, index as u64, *input_root);
            let partition = WorkloadPartition {
                partition_id: partition_root,
                package_root,
                partition_root,
                assigned_operator: [0u8; 32],
                execution_window,
            };
            assign_partition(partition, operators)
        })
        .collect()
}
