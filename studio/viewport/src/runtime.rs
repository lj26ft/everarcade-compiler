#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeProjection { pub projection_hash: String, pub authority_mutation: bool }
pub fn project_runtime_state(state_root: &str, tick: u64) -> RuntimeProjection { RuntimeProjection { projection_hash: crate::stable_hash(&["runtime-projection", state_root, &tick.to_string()]), authority_mutation: false } }
