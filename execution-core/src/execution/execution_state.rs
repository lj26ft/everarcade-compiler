use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExecutionState {
    pub applied_nodes: Vec<String>,
    pub values: BTreeMap<String, Vec<u8>>,
}
