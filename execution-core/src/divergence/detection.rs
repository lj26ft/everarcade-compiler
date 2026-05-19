use serde::{Deserialize, Serialize};

use super::fork::{ContinuityFork, Hash256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DivergenceDetectionReport {
    pub divergence_detected: bool,
    pub shared_ancestor_found: bool,
}

pub fn detect_divergence(
    finalized_checkpoint_a: Hash256,
    finalized_checkpoint_b: Hash256,
    continuity_root_a: Hash256,
    continuity_root_b: Hash256,
    shared_ancestor: Option<Hash256>,
) -> DivergenceDetectionReport {
    let conflicting_finalized = finalized_checkpoint_a != finalized_checkpoint_b;
    let same_ancestry = shared_ancestor.is_some();
    let different_roots = continuity_root_a != continuity_root_b;
    DivergenceDetectionReport {
        divergence_detected: conflicting_finalized && same_ancestry && different_roots,
        shared_ancestor_found: same_ancestry,
    }
}

pub fn build_continuity_fork(
    checkpoint_a: Hash256,
    checkpoint_b: Hash256,
    divergence_tick: u64,
    shared_ancestor: Hash256,
) -> ContinuityFork {
    ContinuityFork {
        checkpoint_a,
        checkpoint_b,
        divergence_tick,
        shared_ancestor,
    }
}
