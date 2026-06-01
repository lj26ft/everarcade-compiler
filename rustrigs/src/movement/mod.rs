use crate::RustrigDescriptor;
use contract_api::protocol_records::{fields, EntityRecord, ProtocolRecord, WorldRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MovementInput {
    pub actor: String,
    pub world: String,
    pub x: i64,
    pub y: i64,
    pub min: i64,
    pub max: i64,
    pub tick: u64,
}
fn w(action: &str, i: &MovementInput) -> WorldRecord {
    WorldRecord::new(
        action,
        i.actor.clone(),
        fields(&[
            ("world", i.world.clone()),
            ("x", i.x.to_string()),
            ("y", i.y.to_string()),
            ("min", i.min.to_string()),
            ("max", i.max.to_string()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn e(action: &str, i: &MovementInput) -> EntityRecord {
    EntityRecord::new(
        action,
        i.actor.clone(),
        fields(&[
            ("x", i.x.to_string()),
            ("y", i.y.to_string()),
            (
                "within_bounds",
                (i.x >= i.min && i.x <= i.max && i.y >= i.min && i.y <= i.max).to_string(),
            ),
            ("tick", i.tick.to_string()),
        ]),
    )
}
pub fn validate_bounds(i: MovementInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::Entity(e("validate-bounds", &i))]
}
pub fn move_actor(i: MovementInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("move-actor", &i)),
        ProtocolRecord::Entity(e("actor-moved", &i)),
    ]
}
pub fn teleport_actor(i: MovementInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("teleport-actor", &i)),
        ProtocolRecord::Entity(e("actor-teleported", &i)),
    ]
}
pub fn descriptors() -> Vec<RustrigDescriptor> {
    ["MoveActor", "ValidateBounds", "TeleportActor"]
        .into_iter()
        .map(|n| RustrigDescriptor::new(n, "1.0.0", "WorldRecord,EntityRecord"))
        .collect()
}
