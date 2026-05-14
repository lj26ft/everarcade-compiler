use sha2::{Digest, Sha256};

use crate::distributed_execution::workload_partition::{Hash, WorkloadPartition};
use crate::distributed_receipts::receipt_store::DistributedExecutionReceipt;

pub fn execute_partition(
    partition: &WorkloadPartition,
    input_root: Hash,
    replay_root: Hash,
) -> DistributedExecutionReceipt {
    let mut hasher = Sha256::new();
    hasher.update(partition.partition_root);
    hasher.update(partition.package_root);
    hasher.update(input_root);
    let execution_root: Hash = hasher.finalize().into();
    DistributedExecutionReceipt::new(partition.partition_root, execution_root, replay_root)
}
