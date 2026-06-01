use crate::RustrigDescriptor;
use contract_api::protocol_records::{
    fields, InventoryRecord, ProtocolRecord, QuestRecord, WorldRecord,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InteractionInput {
    pub actor: String,
    pub object: String,
    pub trigger: String,
    pub item: String,
    pub quest: String,
    pub tick: u64,
}
fn w(action: &str, i: &InteractionInput) -> WorldRecord {
    WorldRecord::new(
        action,
        i.object.clone(),
        fields(&[
            ("actor", i.actor.clone()),
            ("trigger", i.trigger.clone()),
            ("item", i.item.clone()),
            ("quest", i.quest.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn inv(action: &str, i: &InteractionInput) -> InventoryRecord {
    InventoryRecord::new(
        action,
        i.actor.clone(),
        fields(&[
            ("object", i.object.clone()),
            ("item", i.item.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn q(action: &str, i: &InteractionInput) -> QuestRecord {
    QuestRecord::new(
        action,
        i.quest.clone(),
        fields(&[
            ("actor", i.actor.clone()),
            ("object", i.object.clone()),
            ("trigger", i.trigger.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
pub fn interact(i: InteractionInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("interact", &i)),
        ProtocolRecord::Quest(q("interaction-observed", &i)),
    ]
}
pub fn activate_trigger(i: InteractionInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("activate-trigger", &i)),
        ProtocolRecord::Quest(q("trigger-activated", &i)),
    ]
}
pub fn open_container(i: InteractionInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("open-container", &i)),
        ProtocolRecord::Inventory(inv("container-opened", &i)),
    ]
}
pub fn use_object(i: InteractionInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("use-object", &i)),
        ProtocolRecord::Inventory(inv("object-used", &i)),
        ProtocolRecord::Quest(q("object-used", &i)),
    ]
}
pub fn descriptors() -> Vec<RustrigDescriptor> {
    ["Interact", "ActivateTrigger", "OpenContainer", "UseObject"]
        .into_iter()
        .map(|n| RustrigDescriptor::new(n, "1.0.0", "WorldRecord,InventoryRecord,QuestRecord"))
        .collect()
}
