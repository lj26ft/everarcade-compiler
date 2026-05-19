use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{
    errors::DivergenceError,
    fork::{hash_continuity_fork, ContinuityFork, Hash256},
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DivergenceRegistry {
    pub active_forks: BTreeMap<Hash256, ContinuityFork>,
}

pub fn register_divergence(
    registry: &mut DivergenceRegistry,
    fork: ContinuityFork,
) -> Result<Hash256, DivergenceError> {
    let fork_hash = hash_continuity_fork(&fork);
    if registry.active_forks.contains_key(&fork_hash) {
        return Err(DivergenceError::DuplicateFork);
    }
    registry.active_forks.insert(fork_hash, fork);
    Ok(fork_hash)
}
