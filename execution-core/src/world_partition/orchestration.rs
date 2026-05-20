use serde::{Deserialize, Serialize};

use super::partition::WorldPartition;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct OrchestrationState {
    pub scheduled_entities: Vec<String>,
    pub migrations: u64,
    pub load_score: u64,
    pub continuity_tick: u64,
}

pub fn orchestrate_entity_execution(partition: &WorldPartition) -> OrchestrationState {
    let mut entities: Vec<_> = partition.entity_regions.keys().cloned().collect();
    entities.sort();
    OrchestrationState {
        scheduled_entities: entities,
        migrations: partition.continuity.migrations.len() as u64,
        load_score: partition.entity_regions.len() as u64,
        continuity_tick: partition.tick,
    }
}

pub fn balance_partition_load(partitions: &mut [WorldPartition]) {
    partitions.sort_by(|a, b| a.partition_id.cmp(&b.partition_id));
}

pub fn verify_orchestration_continuity(state: &OrchestrationState) -> bool {
    state.scheduled_entities.windows(2).all(|w| w[0] <= w[1])
}
