use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, EntityRecord, ProtocolRecord, WorldRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldInput {
    pub entity: String,
    pub world: String,
    pub x: i64,
    pub y: i64,
    pub owner: String,
    pub faction: String,
    pub tick: u64,
}
fn w(action: &str, i: &WorldInput) -> WorldRecord {
    WorldRecord::new(
        action,
        i.entity.clone(),
        fields(&[
            ("world", i.world.clone()),
            ("x", i.x.to_string()),
            ("y", i.y.to_string()),
            ("owner", i.owner.clone()),
            ("faction", i.faction.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn ent(action: &str, i: &WorldInput) -> EntityRecord {
    EntityRecord::new(
        action,
        i.entity.clone(),
        fields(&[
            ("world", i.world.clone()),
            ("x", i.x.to_string()),
            ("y", i.y.to_string()),
            ("owner", i.owner.clone()),
            ("faction", i.faction.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
macro_rules! rig {
    ($name:ident,$action:literal) => {
        pub struct $name;
        impl Rustrig for $name {
            type Input = WorldInput;
            type Output = WorldRecord;
            fn execute(input: Self::Input) -> Self::Output {
                w($action, &input)
            }
        }
        impl ReplaySafeRustrig for $name {}
        impl VersionedRustrig for $name {
            const NAME: &'static str = stringify!($name);
            const VERSION: &'static str = "1.0.0";
            const RECORD_TYPE: &'static str = "WorldRecord";
        }
    };
}
rig!(SpawnEntity, "spawn-entity");
rig!(DespawnEntity, "despawn-entity");
rig!(MoveEntity, "move-entity");
rig!(AssignFaction, "assign-faction");
rig!(TransferOwnership, "transfer-ownership");
pub fn spawn_entity(i: WorldInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("spawn-entity", &i)),
        ProtocolRecord::Entity(ent("entity-spawned", &i)),
    ]
}
pub fn despawn_entity(i: WorldInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("despawn-entity", &i)),
        ProtocolRecord::Entity(ent("entity-despawned", &i)),
    ]
}
pub fn move_entity(i: WorldInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("move-entity", &i)),
        ProtocolRecord::Entity(ent("entity-moved", &i)),
    ]
}
pub fn assign_faction(i: WorldInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("assign-faction", &i)),
        ProtocolRecord::Entity(ent("faction-assigned", &i)),
    ]
}
pub fn transfer_ownership(i: WorldInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("transfer-ownership", &i)),
        ProtocolRecord::Entity(ent("ownership-transferred", &i)),
    ]
}
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "SpawnEntity",
        "DespawnEntity",
        "MoveEntity",
        "AssignFaction",
        "TransferOwnership",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "WorldRecord,EntityRecord"))
    .collect()
}
