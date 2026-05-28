use super::{runtime::InventoryRuntime, validation::validate_inventory_equivalence};
pub fn restore_inventory(
    checkpoint: &InventoryRuntime,
    replay: &InventoryRuntime,
) -> Result<InventoryRuntime, &'static str> {
    validate_inventory_equivalence(checkpoint, replay)?;
    Ok(replay.clone())
}
