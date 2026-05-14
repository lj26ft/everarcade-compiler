use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineageEvent {
    pub label: String,
    pub payload_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineageTransition {
    pub prior_root: String,
    pub next_root: String,
    pub event: LineageEvent,
}

pub fn advance_lineage(prior_root: &str, label: &str, payload: &[u8]) -> LineageTransition {
    let event = LineageEvent {
        label: label.to_string(),
        payload_hash: hash_bytes(payload),
    };
    let next_root =
        hash_bytes(format!("{}:{}:{}", prior_root, event.label, event.payload_hash).as_bytes());
    LineageTransition {
        prior_root: prior_root.to_string(),
        next_root,
        event,
    }
}
