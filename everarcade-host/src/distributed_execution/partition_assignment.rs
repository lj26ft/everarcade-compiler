use sha2::{Digest, Sha256};

use super::workload_partition::{Hash, WorkloadPartition};

pub fn assign_partition(mut partition: WorkloadPartition, operators: &[Hash]) -> WorkloadPartition {
    let assigned_operator = if operators.is_empty() {
        [0u8; 32]
    } else {
        let mut hasher = Sha256::new();
        hasher.update(partition.partition_id);
        let digest: Hash = hasher.finalize().into();
        operators[digest[0] as usize % operators.len()]
    };
    partition.assigned_operator = assigned_operator;
    partition
}
