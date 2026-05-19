use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationBoundary {
    pub reconciliation_allowed: bool,
}

pub fn verify_reconciliation_boundary(boundary: &ReconciliationBoundary) -> bool {
    !boundary.reconciliation_allowed
}
