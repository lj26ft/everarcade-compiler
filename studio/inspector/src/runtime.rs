pub fn inspect_runtime(runtime_root: &str, replay_root: &str) -> String { crate::stable_hash(&["inspect-runtime", runtime_root, replay_root]) }
pub fn request_direct_mutation(requested: bool) -> Result<(), &'static str> { if requested { Err("inspector writes must route through deterministic editor actions") } else { Ok(()) } }
