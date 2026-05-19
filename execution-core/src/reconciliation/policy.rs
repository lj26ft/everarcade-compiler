use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationPolicy {
    pub quarantine_required: bool,
}

pub fn verify_reconciliation_policy(policy: &ReconciliationPolicy, has_quarantine: bool) -> bool {
    !policy.quarantine_required || has_quarantine
}
