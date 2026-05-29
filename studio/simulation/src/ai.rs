pub fn ai_execution(agent_id: &str, tick: u64) -> String { crate::stable_hash(&["ai-execution", agent_id, &tick.to_string()]) }
