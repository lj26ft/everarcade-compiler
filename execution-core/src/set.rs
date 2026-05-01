use crate::{Contract, State, StateChange};
use std::collections::BTreeMap;

pub struct SetContract;

impl Contract for SetContract {
    fn execute(
        &self,
        payload: &BTreeMap<String, String>,
        state: &mut State,
        changes: &mut Vec<StateChange>,
    ) {
        let key = payload.get("key").unwrap().clone();
        let value = payload.get("value").unwrap().clone();

        let before = state.get(&key).cloned().unwrap_or_default();

        state.insert(key.clone(), value.clone());

        changes.push(StateChange {
            key,
            before,
            after: value,
        });
    }
}
