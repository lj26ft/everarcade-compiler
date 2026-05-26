use execution_core::render_bridge::ProjectedInventoryFrame;

pub fn render_inventory_view(inventory: &[ProjectedInventoryFrame]) -> String {
    inventory
        .iter()
        .map(|i| format!("owner={} root={}", i.owner, i.inventory_root))
        .collect::<Vec<_>>()
        .join(" | ")
}
