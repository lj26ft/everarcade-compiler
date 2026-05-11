use serde::{Deserialize, Serialize};

use super::aggregation::AggregateProof;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCheckpoint {
    pub epoch_id: u64,
    pub aggregate_root: String,
    pub settlement_anchor: String,
}

pub fn checkpoint_root(aggregate: &AggregateProof) -> String {
    crate::hashing::hash_bytes(aggregate.aggregate_hash.as_bytes())
}
