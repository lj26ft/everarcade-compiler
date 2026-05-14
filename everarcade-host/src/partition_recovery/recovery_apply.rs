use super::recovery_plan::RecoveryPlan;

pub fn apply_recovery_plan(plan: &RecoveryPlan) -> bool {
    plan.requires_checkpoint_import || plan.missing_receipts == 0
}
