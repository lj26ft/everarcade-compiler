use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, WorldRecord};
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
fn rec(action: &str, i: WorldInput) -> WorldRecord {
    WorldRecord::new(
        action,
        i.entity,
        fields(&[
            ("world", i.world),
            ("x", i.x.to_string()),
            ("y", i.y.to_string()),
            ("owner", i.owner),
            ("faction", i.faction),
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
                rec($action, input)
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
rig!(TransferOwnership, "transfer-ownership");
rig!(AssignFaction, "assign-faction");
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "SpawnEntity",
        "DespawnEntity",
        "MoveEntity",
        "TransferOwnership",
        "AssignFaction",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "WorldRecord"))
    .collect()
}
