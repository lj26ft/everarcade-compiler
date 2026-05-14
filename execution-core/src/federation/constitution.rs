use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstitutionState {
    pub constitutional_root: String,
    pub amendment_history: Vec<String>,
}

impl ConstitutionState {
    pub fn genesis(seed: &[u8]) -> Self {
        Self {
            constitutional_root: hash_bytes(seed),
            amendment_history: Vec::new(),
        }
    }

    pub fn append_amendment(&self, amendment_id: impl Into<String>) -> Self {
        let amendment_id = amendment_id.into();
        let mut amendment_history = self.amendment_history.clone();
        amendment_history.push(amendment_id.clone());
        let constitutional_root =
            hash_bytes(format!("{}:{}", self.constitutional_root, amendment_id).as_bytes());
        Self {
            constitutional_root,
            amendment_history,
        }
    }
}
