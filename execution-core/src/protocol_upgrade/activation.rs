use super::{transition::UpgradeTransition, validation};

pub fn can_activate_epoch(
    verifier_majority_accepted: bool,
    deterministic_validation_passed: bool,
    settlement_checkpoint_confirmed: bool,
    transition: &UpgradeTransition,
) -> bool {
    verifier_majority_accepted
        && deterministic_validation_passed
        && settlement_checkpoint_confirmed
        && validation::validate_upgrade_safety(transition)
}
