use serde::{Deserialize, Serialize};

use crate::divergence::fork::Hash256;

use super::{descriptor::hash_reconciliation_descriptor, registry::ReconciliationRegistry};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationProof {
    pub fork_hash: Hash256,
    pub quarantine_hash: Hash256,
}

pub fn verify_reconciliation_proof(
    proof: &ReconciliationProof,
    registry: &ReconciliationRegistry,
) -> bool {
    registry
        .quarantined_forks
        .get(&proof.fork_hash)
        .map(|descriptor| proof.quarantine_hash == hash_reconciliation_descriptor(descriptor))
        .unwrap_or(false)
}
