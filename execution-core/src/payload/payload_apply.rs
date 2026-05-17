use crate::state::{ExecutionState, StateDiff, StateInsert, StateRemoval, StateUpdate};

use super::execution_payload::ExecutionPayload;

/// Deterministic payload execution:
/// state + payload -> StateDiff
pub fn execute_payload(payload: &ExecutionPayload, state: &ExecutionState) -> StateDiff {
    let mut diff = StateDiff::default();

    for mutation in payload.mutations() {
        match &mutation.value {
            Some(value) => {
                if state.entries.contains_key(&mutation.key) {
                    diff.updates.push(StateUpdate {
                        key: mutation.key.clone(),
                        value: value.clone(),
                    });
                } else {
                    diff.inserts.push(StateInsert {
                        key: mutation.key.clone(),
                        value: value.clone(),
                    });
                }
            }
            None => {
                if state.entries.contains_key(&mutation.key) {
                    diff.removals.push(StateRemoval {
                        key: mutation.key.clone(),
                    });
                }
            }
        }
    }

    diff.canonicalize();
    diff
}
