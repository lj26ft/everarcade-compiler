use crate::distributed_execution::workload_partition::WorkloadPartition;

pub fn reconcile_partitions(
    local: &[WorkloadPartition],
    remote: &[WorkloadPartition],
) -> Vec<WorkloadPartition> {
    if remote.len() > local.len() {
        remote.to_vec()
    } else {
        local.to_vec()
    }
}
