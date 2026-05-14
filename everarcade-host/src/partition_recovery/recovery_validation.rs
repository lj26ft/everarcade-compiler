use super::recovery_plan::RecoveryPlan;

pub fn validate_recovery_plan(plan: &RecoveryPlan) -> bool {
    !(plan.requires_checkpoint_import && plan.latest_checkpoint_root == [0u8; 32])
}
