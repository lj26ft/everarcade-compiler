use super::transition::UpgradeTransition;

pub fn validate_upgrade_safety(transition: &UpgradeTransition) -> bool {
    transition.snapshot_compatible
        && transition.receipt_compatible
        && transition.deterministic_migration
}
