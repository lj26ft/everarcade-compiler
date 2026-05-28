use serde::{Deserialize, Serialize};

use crate::ai_memory::AiMemoryRuntime;

use super::{decision::AiDecision, planner, scheduler, validation};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AiRuntimeError {
    HiddenMutation,
    NonDeterministicDecision,
    ReplayDivergence,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiRuntime {
    pub tick: u64,
    pub decisions: Vec<AiDecision>,
    pub memory: AiMemoryRuntime,
}

impl AiRuntime {
    pub fn execute(
        &mut self,
        entity_ids: Vec<String>,
        replay_root: &str,
    ) -> Result<Vec<AiDecision>, AiRuntimeError> {
        if replay_root.is_empty() {
            return Err(AiRuntimeError::NonDeterministicDecision);
        }
        let ordered = scheduler::deterministic_ai_order(entity_ids);
        let mut decisions = Vec::new();
        for entity_id in ordered {
            let memory_count = self.memory.store.entries_for(&entity_id).len();
            let action = planner::plan_action(&entity_id, self.tick, memory_count);
            let decision = AiDecision {
                entity_id: entity_id.clone(),
                tick: self.tick,
                action: action.clone(),
                replay_root: replay_root.to_string(),
            };
            self.memory
                .append(&entity_id, &action, replay_root)
                .map_err(|_| AiRuntimeError::HiddenMutation)?;
            self.decisions.push(decision.clone());
            decisions.push(decision);
        }
        if !validation::decisions_are_deterministic(&decisions) {
            return Err(AiRuntimeError::NonDeterministicDecision);
        }
        self.tick += 1;
        Ok(decisions)
    }
}
