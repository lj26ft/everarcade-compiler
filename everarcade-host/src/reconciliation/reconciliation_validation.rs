use super::reconciliation_plan::ReconciliationPlan;

pub fn validate_reconciliation_plan(plan: &ReconciliationPlan) -> bool {
    !plan.selected_peer_id.is_empty()
}
