use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::divergence::fork::Hash256;

use super::{descriptor::ReconciliationDescriptor, errors::ReconciliationError};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationRegistry {
    pub quarantined_forks: BTreeMap<Hash256, ReconciliationDescriptor>,
}

pub fn register_quarantined_fork(
    registry: &mut ReconciliationRegistry,
    descriptor: ReconciliationDescriptor,
) -> Result<Hash256, ReconciliationError> {
    let fork_hash = descriptor.fork_hash;
    if registry.quarantined_forks.contains_key(&fork_hash) {
        return Err(ReconciliationError::DuplicateFork);
    }
    registry.quarantined_forks.insert(fork_hash, descriptor);
    Ok(fork_hash)
}
