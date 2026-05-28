use serde::{Deserialize, Serialize};

use super::node::{BehaviorNode, BehaviorStatus};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BehaviorExecution {
    pub tick: u64,
    pub node_id: String,
    pub status: BehaviorStatus,
    pub replay_root: String,
}

pub fn execute_node(tick: u64, node: &BehaviorNode, replay_root: &str) -> BehaviorExecution {
    BehaviorExecution {
        tick,
        node_id: node.id.clone(),
        status: node.status.clone(),
        replay_root: replay_root.to_string(),
    }
}
