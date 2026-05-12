use super::pruning_policy::PruningPolicy;

pub fn retain_proof_commitments(policy: &PruningPolicy) -> bool {
    policy.require_proof_commitment
}
