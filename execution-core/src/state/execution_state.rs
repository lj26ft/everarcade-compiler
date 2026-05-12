use std::collections::BTreeMap;

pub type StateValue = Vec<u8>;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExecutionState {
    pub revision: u64,
    pub entries: BTreeMap<String, StateValue>,
}

impl ExecutionState {
    pub fn with_revision(mut self, revision: u64) -> Self {
        self.revision = revision;
        self
    }
}
