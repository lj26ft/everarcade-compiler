use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationBoundary {
    pub automatic_reconciliation_disabled: bool,
}

pub fn verify_reconciliation_boundary(boundary: &ReconciliationBoundary) -> bool {
    boundary.automatic_reconciliation_disabled
}
