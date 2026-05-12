use super::treaty::{ExecutionTreaty, Hash};

pub fn treaty_root(treaty: &ExecutionTreaty) -> Hash {
    let mut out = treaty.constitutional_scope_root;
    for (i, b) in treaty.capability_scope_root.iter().enumerate() { out[i] ^= *b; }
    for (i, b) in treaty.arbitration_root.iter().enumerate() { out[i] ^= *b; }
    out
}
