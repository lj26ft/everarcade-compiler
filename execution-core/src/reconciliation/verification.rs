use serde::{Deserialize, Serialize};

use crate::divergence::fork::Hash256;

use super::registry::ReconciliationRegistry;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationVerificationReport {
    pub quarantined: bool,
    pub reconciliation_prohibited: bool,
}

pub fn verify_reconciliation(
    fork_hash: Hash256,
    registry: &ReconciliationRegistry,
) -> ReconciliationVerificationReport {
    let quarantined = registry.quarantined_forks.contains_key(&fork_hash);
    let reconciliation_prohibited = registry
        .quarantined_forks
        .get(&fork_hash)
        .map(|d| !d.reconciliation_allowed)
        .unwrap_or(false);
    ReconciliationVerificationReport {
        quarantined,
        reconciliation_prohibited,
    }
}
