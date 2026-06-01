use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, EntityRecord, InventoryRecord, ProtocolRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventoryInput {
    pub owner: String,
    pub item: String,
    pub quantity: u64,
    pub counterparty: String,
    pub slot: String,
    pub tick: u64,
}
fn inv(action: &str, i: &InventoryInput) -> InventoryRecord {
    InventoryRecord::new(
        action,
        i.owner.clone(),
        fields(&[
            ("item", i.item.clone()),
            ("quantity", i.quantity.to_string()),
            ("counterparty", i.counterparty.clone()),
            ("slot", i.slot.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn ent(action: &str, i: &InventoryInput) -> EntityRecord {
    EntityRecord::new(
        action,
        i.owner.clone(),
        fields(&[
            ("item", i.item.clone()),
            ("quantity", i.quantity.to_string()),
            ("slot", i.slot.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
macro_rules! rig {
    ($name:ident,$action:literal) => {
        pub struct $name;
        impl Rustrig for $name {
            type Input = InventoryInput;
            type Output = InventoryRecord;
            fn execute(input: Self::Input) -> Self::Output {
                inv($action, &input)
            }
        }
        impl ReplaySafeRustrig for $name {}
        impl VersionedRustrig for $name {
            const NAME: &'static str = stringify!($name);
            const VERSION: &'static str = "1.0.0";
            const RECORD_TYPE: &'static str = "InventoryRecord";
        }
    };
}
rig!(AddItem, "add-item");
rig!(RemoveItem, "remove-item");
rig!(TransferItem, "transfer-item");
rig!(EquipItem, "equip-item");
rig!(UnequipItem, "unequip-item");
rig!(StackItem, "stack-item");
rig!(SplitItem, "split-item");
pub fn add_item(i: InventoryInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Inventory(inv("add-item", &i)),
        ProtocolRecord::Entity(ent("inventory-added", &i)),
    ]
}
pub fn remove_item(i: InventoryInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Inventory(inv("remove-item", &i)),
        ProtocolRecord::Entity(ent("inventory-removed", &i)),
    ]
}
pub fn transfer_item(i: InventoryInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Inventory(inv("transfer-item", &i)),
        ProtocolRecord::Entity(ent("inventory-transferred", &i)),
    ]
}
pub fn equip_item(i: InventoryInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Inventory(inv("equip-item", &i)),
        ProtocolRecord::Entity(ent("item-equipped", &i)),
    ]
}
pub fn unequip_item(i: InventoryInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Inventory(inv("unequip-item", &i)),
        ProtocolRecord::Entity(ent("item-unequipped", &i)),
    ]
}
pub fn stack_item(i: InventoryInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Inventory(inv("stack-item", &i)),
        ProtocolRecord::Entity(ent("item-stacked", &i)),
    ]
}
pub fn split_item(i: InventoryInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Inventory(inv("split-item", &i)),
        ProtocolRecord::Entity(ent("item-split", &i)),
    ]
}
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "AddItem",
        "RemoveItem",
        "TransferItem",
        "EquipItem",
        "UnequipItem",
        "StackItem",
        "SplitItem",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "InventoryRecord,EntityRecord"))
    .collect()
}
