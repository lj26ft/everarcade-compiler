use crate::stable_hash;

pub fn inspect_execution_state(tick: u64, state_root: &str) -> String { stable_hash(&["entity-runtime", &tick.to_string(), state_root]) }
