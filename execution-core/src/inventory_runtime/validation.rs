use super::runtime::InventoryRuntime;
pub fn validate_inventory(i: &InventoryRuntime) -> bool {
    i.records
        .last()
        .map(|r| i.continuity_root == format!("inventory:continuity:{}", r.ownership_root))
        .unwrap_or(false)
        && i.records
            .windows(2)
            .all(|w| w[1].previous_owner == w[0].owner_id)
}
pub fn validate_inventory_equivalence(
    a: &InventoryRuntime,
    b: &InventoryRuntime,
) -> Result<(), &'static str> {
    if a == b && validate_inventory(a) {
        Ok(())
    } else {
        Err("inventory replay divergence rejected")
    }
}
pub fn reject_ownership_mutation(authorized: bool) -> Result<(), &'static str> {
    if authorized {
        Ok(())
    } else {
        Err("invalid ownership mutation rejected")
    }
}
