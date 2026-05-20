use super::{partition::WorldPartition, verification};

pub fn replay_partition_timeline(partition: &WorldPartition) -> Vec<String> {
    let mut lineage = partition.continuity.lineage.clone();
    lineage.sort();
    lineage
}

pub fn verify_partition_convergence(partitions: &[WorldPartition]) -> bool {
    partitions
        .windows(2)
        .all(|w| replay_partition_timeline(&w[0]) == replay_partition_timeline(&w[1]))
}

pub fn reconstruct_world_partitions(partitions: &[WorldPartition]) -> bool {
    partitions
        .iter()
        .all(|p| verification::verify_partition_interaction(&p.event_sequences))
}
