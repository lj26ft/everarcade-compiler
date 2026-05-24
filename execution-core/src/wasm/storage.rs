use std::collections::BTreeMap;

use super::mutations::ExecutionMutationSet;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HostOwnedState {
    pub data: BTreeMap<String, Vec<u8>>,
}

impl HostOwnedState {
    pub fn apply_mutations(&self, mutations: &ExecutionMutationSet) -> Self {
        let mut next = self.clone();
        for (k, v) in &mutations.entries {
            next.data.insert(k.clone(), v.clone());
        }
        next
    }
}
