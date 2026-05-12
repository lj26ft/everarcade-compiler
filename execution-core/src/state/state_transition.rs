use crate::payload::ExecutionPayload;
use crate::receipt_runtime::execution_receipt::ExecutionReceipt;

use super::execution_state::ExecutionState;
use super::state_diff::{StateDiff, StateInsert, StateRemoval, StateUpdate};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionResult {
    pub next: ExecutionState,
    pub diff: StateDiff,
    pub receipt_hash: [u8; 32],
}

pub fn apply_execution_transition(
    previous: ExecutionState,
    receipt: ExecutionReceipt,
    payload: ExecutionPayload,
) -> TransitionResult {
    let mut next = previous.clone();
    let mut diff = StateDiff::default();
    let mut mutations = payload.mutations().to_vec();
    mutations.sort();

    for mutation in mutations {
        match mutation.value {
            Some(value) => {
                if next.entries.contains_key(&mutation.key) {
                    diff.updates.push(StateUpdate { key: mutation.key.clone(), value: value.clone() });
                } else {
                    diff.inserts.push(StateInsert { key: mutation.key.clone(), value: value.clone() });
                }
                next.entries.insert(mutation.key, value);
            }
            None => {
                if next.entries.remove(&mutation.key).is_some() {
                    diff.removals.push(StateRemoval { key: mutation.key });
                }
            }
        }
    }

    diff.canonicalize();
    next.revision = next.revision.saturating_add(1);
    TransitionResult { next, diff, receipt_hash: receipt.receipt_hash() }
}
