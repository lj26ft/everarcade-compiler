use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PartitionWork {
    pub partition_id: String,
    pub ecs_entities: u64,
    pub ai_entities: u64,
}
