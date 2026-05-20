use std::collections::BTreeMap;

pub type RuntimeStateMap = BTreeMap<Vec<u8>, Vec<u8>>;

pub fn canonicalize_state(state: &RuntimeStateMap) -> Vec<(Vec<u8>, Vec<u8>)> {
    state.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}
