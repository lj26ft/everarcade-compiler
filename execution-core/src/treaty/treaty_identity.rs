use super::treaty::{ExecutionTreaty, Hash};

pub fn treaty_identity_hash(treaty: &ExecutionTreaty) -> Hash {
    let mut out = treaty.treaty_id;
    for domain in &treaty.participating_domains {
        for (i, b) in domain.iter().enumerate() { out[i] ^= *b; }
    }
    out
}
