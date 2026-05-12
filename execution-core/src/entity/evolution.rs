use crate::hashing::hash_bytes;

pub fn evolve_deterministically(state_root: &str, rule_id: &str) -> String {
    hash_bytes(format!("evolve:{state_root}:{rule_id}").as_bytes())
}
