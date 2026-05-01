use std::collections::BTreeMap;

use crate::{
    contract::Contract,
    state::{State, StateChange},
};

pub struct SetContract;

impl Contract for SetContract {
    fn execute(
        &self,
        payload: &BTreeMap<String, String>,
        state: &mut State,
        changes: &mut Vec<StateChange>,
    ) {
        let key = payload.get("key").unwrap().to_string();
        let value = payload.get("value").unwrap().to_string();

        let before = state.get(&key).cloned().unwrap_or_default();

        state.insert(key.clone(), value.clone());

        changes.push(StateChange {
            key,
            before,
            after: value,
        });
    }
}
