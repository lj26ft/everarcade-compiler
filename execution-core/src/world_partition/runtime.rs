use serde::{Deserialize, Serialize};

use super::{partition::WorldPartition, validation};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorldPartitionError {
    PartitionDivergence,
    UnsynchronizedMutation,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldPartitionRuntime {
    pub partitions: Vec<WorldPartition>,
    pub replay_roots: Vec<String>,
}

impl WorldPartitionRuntime {
    pub fn add_partition(
        &mut self,
        partition: WorldPartition,
        replay_root: &str,
    ) -> Result<(), WorldPartitionError> {
        if replay_root.is_empty() {
            return Err(WorldPartitionError::UnsynchronizedMutation);
        }
        self.partitions.push(partition);
        self.partitions
            .sort_by(|a, b| a.partition_id.cmp(&b.partition_id));
        self.replay_roots.push(replay_root.to_string());
        Ok(())
    }
    pub fn validate(&self) -> Result<(), WorldPartitionError> {
        if validation::partition_runtime_is_deterministic(self) {
            Ok(())
        } else {
            Err(WorldPartitionError::PartitionDivergence)
        }
    }
}
