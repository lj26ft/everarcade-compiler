use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionMutationSet {
    pub entries: Vec<(String, Vec<u8>)>,
}

impl ExecutionMutationSet {
    pub fn reject_duplicates(&self) -> bool {
        let mut keys = self.entries.iter().map(|(k, _)| k).collect::<Vec<_>>();
        keys.sort();
        keys.dedup();
        keys.len() == self.entries.len()
    }
}
