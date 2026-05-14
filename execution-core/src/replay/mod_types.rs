use crate::hashing::hash_bytes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceStep {
    pub receipt_hash: String,
    pub prior_state_root: String,
    pub next_state_root: String,
    pub transition_root: String,
    pub diff_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayResult {
    pub steps: Vec<TraceStep>,
    pub divergence: Option<usize>,
}

pub fn replay_receipt_chain(receipts: &[(&str, Option<&str>, &str)]) -> ReplayResult {
    let mut steps: Vec<TraceStep> = Vec::new();
    let mut divergence = None;
    for (i, (id, parent, state_root)) in receipts.iter().enumerate() {
        if i > 0 && Some(receipts[i - 1].0) != *parent {
            divergence = Some(i);
            break;
        }
        let prior = if i == 0 {
            hash_bytes(b"genesis")
        } else {
            steps[i - 1].next_state_root.clone()
        };
        let diff_root = hash_bytes(id.as_bytes());
        let next = hash_bytes([prior.as_bytes(), diff_root.as_bytes()].concat().as_slice());
        if next != *state_root {
            divergence = Some(i);
        }
        let transition = hash_bytes([prior.as_bytes(), next.as_bytes()].concat().as_slice());
        steps.push(TraceStep {
            receipt_hash: hash_bytes(id.as_bytes()),
            prior_state_root: prior,
            next_state_root: next,
            transition_root: transition,
            diff_root,
        });
        if divergence.is_some() {
            break;
        }
    }
    ReplayResult { steps, divergence }
}
