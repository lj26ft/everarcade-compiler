use serde::{Deserialize, Serialize};

use crate::divergence::fork::Hash256;

use super::registry::ReconciliationRegistry;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuarantinedFork {
    pub fork_hash: Hash256,
    pub quarantined: bool,
}

pub fn verify_quarantine(quarantine: &QuarantinedFork, registry: &ReconciliationRegistry) -> bool {
    quarantine.quarantined
        && registry
            .quarantined_forks
            .contains_key(&quarantine.fork_hash)
}
