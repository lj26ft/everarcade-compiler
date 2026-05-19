use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DivergencePolicy {
    pub reject_conflicting_finality: bool,
}

pub fn verify_divergence_policy(policy: &DivergencePolicy, conflicting_finality: bool) -> bool {
    !conflicting_finality || policy.reject_conflicting_finality
}
