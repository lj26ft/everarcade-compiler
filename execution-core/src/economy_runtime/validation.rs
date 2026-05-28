use super::runtime::EconomyRuntime;
pub fn validate_economy(e: &EconomyRuntime) -> bool {
    e.continuity_root == format!("economy:continuity:{}:{}", e.tick, e.ledger_root)
        && e.ledger
            .windows(2)
            .all(|w| w[1].previous_root == w[0].entry_root)
}
pub fn validate_ledger_equivalence(
    a: &EconomyRuntime,
    b: &EconomyRuntime,
) -> Result<(), &'static str> {
    if a == b && validate_economy(a) {
        Ok(())
    } else {
        Err("ledger divergence rejected")
    }
}
pub fn reject_settlement_mutation(authorized: bool) -> Result<(), &'static str> {
    if authorized {
        Ok(())
    } else {
        Err("unauthorized settlement mutation rejected")
    }
}
