use crate::RustrigDescriptor;
use contract_api::protocol_records::{fields, EconomyRecord, InventoryRecord, ProtocolRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CraftingInput {
    pub actor: String,
    pub recipe: String,
    pub input_item: String,
    pub output_item: String,
    pub quantity: u64,
    pub tick: u64,
}
fn inv(action: &str, i: &CraftingInput) -> InventoryRecord {
    InventoryRecord::new(
        action,
        i.actor.clone(),
        fields(&[
            ("recipe", i.recipe.clone()),
            ("input_item", i.input_item.clone()),
            ("output_item", i.output_item.clone()),
            ("quantity", i.quantity.to_string()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn econ(action: &str, i: &CraftingInput) -> EconomyRecord {
    EconomyRecord::new(
        action,
        i.actor.clone(),
        fields(&[
            ("recipe", i.recipe.clone()),
            ("quantity", i.quantity.to_string()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
pub fn validate_recipe(i: CraftingInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::Inventory(inv("validate-recipe", &i))]
}
pub fn consume_inputs(i: CraftingInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Inventory(inv("consume-inputs", &i)),
        ProtocolRecord::Economy(econ("crafting-cost", &i)),
    ]
}
pub fn produce_outputs(i: CraftingInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Inventory(inv("produce-outputs", &i)),
        ProtocolRecord::Economy(econ("crafting-output", &i)),
    ]
}
pub fn craft_item(i: CraftingInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Inventory(inv("consume-inputs", &i)),
        ProtocolRecord::Inventory(inv("produce-outputs", &i)),
        ProtocolRecord::Economy(econ("craft-item", &i)),
    ]
}
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "ValidateRecipe",
        "ConsumeInputs",
        "ProduceOutputs",
        "CraftItem",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "InventoryRecord,EconomyRecord"))
    .collect()
}
