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
    pub fn derive_root(m: &ExecutionMutationSet) -> PersistentStateRoot {
        PersistentStateRoot(hex::encode(sha256(
            &serde_json::to_vec(m).unwrap_or_default(),
        )))
    }
}
