use super::{checkpoint_retention::retain_checkpoints, proof_retention::retain_proof_commitments, pruning_policy::PruningPolicy};

pub fn validate_pruning_policy(policy: &PruningPolicy) -> bool {
    retain_checkpoints(policy) && retain_proof_commitments(policy)
}
