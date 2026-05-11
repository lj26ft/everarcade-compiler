use serde::{Deserialize, Serialize};

use super::aggregation::AggregateProof;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecursiveProofLineage {
    pub depth: u64,
    pub lineage_hash: String,
}

pub fn compose_lineage(aggregate: &AggregateProof, parent_hash: &str) -> RecursiveProofLineage {
    let lineage_hash = crate::hashing::hash_bytes(format!("{}:{}", parent_hash, aggregate.aggregate_hash).as_bytes());
    RecursiveProofLineage { depth: 1, lineage_hash }
}
