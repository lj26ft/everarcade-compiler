use everarcade_abi::{State, StateChange};

pub fn apply_state_diff(state: &mut State, state_changes: &[StateChange]) {
    for change in state_changes {
        state.insert(change.key.clone(), change.after.clone());
    }
}
