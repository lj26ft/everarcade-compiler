use super::runtime::CivilizationRuntimeState;
pub fn validate_civilization_runtime(s: &CivilizationRuntimeState) -> bool {
    s.continuity_root
        == format!(
            "civilization:{}:continuity:{}:{}:{}",
            s.civilization_id, s.tick, s.economy.ledger_root, s.governance.governance_root
        )
}
pub fn validate_civilization_equivalence(
    a: &CivilizationRuntimeState,
    b: &CivilizationRuntimeState,
) -> Result<(), &'static str> {
    if a == b && validate_civilization_runtime(a) {
        Ok(())
    } else {
        Err("civilization divergence rejected")
    }
}
pub fn reject_replay_authority_mutation(authority_write: bool) -> Result<(), &'static str> {
    if authority_write {
        Err("replay-derived authority mutation rejected")
    } else {
        Ok(())
    }
}
