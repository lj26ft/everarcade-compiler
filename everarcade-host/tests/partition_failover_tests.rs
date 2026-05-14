use everarcade_host::distributed_execution::workload_partition::WorkloadPartition;
use everarcade_host::operator_recovery::partition_failover::failover_partition;

#[test]
fn partition_reassigned_on_failure() {
    let part = WorkloadPartition {
        partition_id: [1; 32],
        package_root: [2; 32],
        partition_root: [1; 32],
        assigned_operator: [9; 32],
        execution_window: [3; 32],
    };
    let reassigned = failover_partition(part, [8; 32]);
    assert_eq!(reassigned.assigned_operator, [8; 32]);
}
