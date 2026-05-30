use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, InventoryRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventoryInput {
    pub owner: String,
    pub item: String,
    pub quantity: u64,
    pub counterparty: String,
    pub slot: String,
    pub tick: u64,
}
fn rec(action: &str, i: InventoryInput) -> InventoryRecord {
    InventoryRecord::new(
        action,
        i.owner,
        fields(&[
            ("item", i.item),
            ("quantity", i.quantity.to_string()),
            ("counterparty", i.counterparty),
            ("slot", i.slot),
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
                rec($action, input)
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
rig!(StackItem, "stack-item");
rig!(SplitItem, "split-item");
rig!(EquipItem, "equip-item");
rig!(UnequipItem, "unequip-item");
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "AddItem",
        "RemoveItem",
        "TransferItem",
        "StackItem",
        "SplitItem",
        "EquipItem",
        "UnequipItem",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "InventoryRecord"))
    .collect()
}
