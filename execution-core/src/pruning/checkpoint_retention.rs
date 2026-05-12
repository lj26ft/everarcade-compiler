use super::pruning_policy::PruningPolicy;

pub fn retain_checkpoints(policy: &PruningPolicy) -> bool {
    policy.require_checkpoint
}
