use serde::{Deserialize, Serialize};

use crate::hashing::sha256;

use super::mutations::ExecutionMutationSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentStateRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateTransitionEnvelope {
    pub prior_root: PersistentStateRoot,
    pub next_root: PersistentStateRoot,
    pub mutations: ExecutionMutationSet,
}

pub struct StatefulExecutionRuntime;
impl StatefulExecutionRuntime {
    pub fn validate_mutations(m: &ExecutionMutationSet) -> anyhow::Result<()> {
        if !m.reject_duplicates() {
            anyhow::bail!("duplicate mutation keys are forbidden");
        }
        Ok(())
    }

    pub fn derive_root(m: &ExecutionMutationSet) -> PersistentStateRoot {
        let mut entries = m.entries.clone();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        PersistentStateRoot(hex::encode(sha256(
            &serde_json::to_vec(&entries).unwrap_or_default(),
        )))
    }
}
