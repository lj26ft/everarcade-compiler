use super::{ExecutionState, StateDiff};

/// Applies a deterministic state diff in canonical phase order:
/// inserts -> updates -> removals -> revision increment.
pub fn apply_state_diff(previous: &ExecutionState, diff: &StateDiff) -> ExecutionState {
    let mut next = previous.clone();

    for insert in &diff.inserts {
        next.entries
            .insert(insert.key.clone(), insert.value.clone());
    }
    for update in &diff.updates {
        next.entries
            .insert(update.key.clone(), update.value.clone());
    }
    for removal in &diff.removals {
        next.entries.remove(&removal.key);
    }

    next.revision = next.revision.saturating_add(1);
    next
}
