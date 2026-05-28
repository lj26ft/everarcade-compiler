use serde::{Deserialize, Serialize};

use super::{
    execution::{execute_node, BehaviorExecution},
    node::BehaviorNode,
    scheduler, validation,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BehaviorTreeError {
    HiddenMutation,
    Divergence,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BehaviorTreeRuntime {
    pub tick: u64,
    pub executions: Vec<BehaviorExecution>,
}

impl BehaviorTreeRuntime {
    pub fn execute(
        &mut self,
        nodes: Vec<BehaviorNode>,
        replay_root: &str,
    ) -> Result<Vec<BehaviorExecution>, BehaviorTreeError> {
        if replay_root.is_empty() {
            return Err(BehaviorTreeError::Divergence);
        }
        let ordered = scheduler::order_nodes(nodes);
        let events: Vec<_> = ordered
            .iter()
            .map(|n| execute_node(self.tick, n, replay_root))
            .collect();
        if !validation::execution_is_ordered(&events) {
            return Err(BehaviorTreeError::Divergence);
        }
        self.executions.extend(events.clone());
        self.tick += 1;
        Ok(events)
    }
}
