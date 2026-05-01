use crate::{Contract, State, StateChange};
use std::collections::BTreeMap;

pub struct IncrementContract;

impl Contract for IncrementContract {
    fn execute(
        &self,
        payload: &BTreeMap<String, String>,
        state: &mut State,
        changes: &mut Vec<StateChange>,
    ) {
        let key = payload.get("key").unwrap().clone();
        let amount: i64 = payload.get("amount").unwrap().parse().unwrap();

        let current = state.get(&key).cloned().unwrap_or("0".to_string());
        let current_num: i64 = current.parse().unwrap();

        let next = (current_num + amount).to_string();

        state.insert(key.clone(), next.clone());

        changes.push(StateChange {
            key,
            before: current,
            after: next,
        });
    }
}
