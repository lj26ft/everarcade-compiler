use crate::distributed_execution::workload_partition::WorkloadPartition;

pub fn requires_recovery(partition: &WorkloadPartition, live_operator: [u8; 32]) -> bool {
    partition.assigned_operator != live_operator
}
