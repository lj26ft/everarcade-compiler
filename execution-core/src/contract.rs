use std::collections::BTreeMap;

use crate::state::{State, StateChange};

pub trait Contract {
    fn execute(
        &self,
        payload: &BTreeMap<String, String>,
        state: &mut State,
        changes: &mut Vec<StateChange>,
    );
}
