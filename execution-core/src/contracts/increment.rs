use std::collections::BTreeMap;

use crate::{
    contract::Contract,
    state::{State, StateChange},
};

pub struct IncrementContract;

impl Contract for IncrementContract {
    fn execute(
        &self,
        payload: &BTreeMap<String, String>,
        state: &mut State,
        changes: &mut Vec<StateChange>,
    ) {
        let key = payload.get("key").unwrap().to_string();

        let amount: i64 = payload
            .get("amount")
            .unwrap()
            .parse()
            .unwrap();

        let before = state.get(&key).cloned().unwrap_or_default();

        let current: i64 =
            before.parse().unwrap_or(0);

        let updated =
            (current + amount).to_string();

        state.insert(key.clone(), updated.clone());

        changes.push(StateChange {
            key,
            before,
            after: updated,
        });
    }
}
