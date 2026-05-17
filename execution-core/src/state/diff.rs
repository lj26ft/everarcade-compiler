use std::collections::BTreeSet;

use everarcade_abi::StateChange;

use super::{
    errors::StateError,
    tree::{CanonicalState, Hash256},
};

pub fn apply_diff(state: &mut CanonicalState, diff: &[StateChange]) -> Result<Hash256, StateError> {
    let mut seen = BTreeSet::new();
    let mut ordered = diff.to_vec();
    ordered.sort_by(|a, b| a.key.as_bytes().cmp(b.key.as_bytes()));

    for change in &ordered {
        let key = change.key.as_bytes().to_vec();
        if !seen.insert(key.clone()) {
            return Err(StateError::DuplicateKey { key });
        }
        let expected = change.before.as_bytes().to_vec();
        let actual = state.entries.get(&key).cloned().unwrap_or_default();
        if expected != actual {
            return Err(StateError::BeforeMismatch {
                key,
                expected,
                actual,
            });
        }

        if change.after.is_empty() {
            state.entries.remove(&key);
        } else {
            state.entries.insert(key, change.after.as_bytes().to_vec());
        }
    }

    Ok(state.root())
}
