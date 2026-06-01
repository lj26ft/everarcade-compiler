use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, CombatRecord, EntityRecord, ProtocolRecord};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombatInput {
    pub actor: String,
    pub target: String,
    pub amount: u64,
    pub status: String,
    pub tick: u64,
}

fn combat(action: &str, i: &CombatInput) -> CombatRecord {
    CombatRecord::new(
        action,
        i.target.clone(),
        fields(&[
            ("actor", i.actor.clone()),
            ("amount", i.amount.to_string()),
            ("status", i.status.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn entity(action: &str, i: &CombatInput) -> EntityRecord {
    EntityRecord::new(
        action,
        i.target.clone(),
        fields(&[
            ("actor", i.actor.clone()),
            ("delta", i.amount.to_string()),
            ("status", i.status.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
macro_rules! rig {
    ($name:ident, $action:literal) => {
        pub struct $name;
        impl Rustrig for $name {
            type Input = CombatInput;
            type Output = CombatRecord;
            fn execute(input: Self::Input) -> Self::Output {
                combat($action, &input)
            }
        }
        impl ReplaySafeRustrig for $name {}
        impl VersionedRustrig for $name {
            const NAME: &'static str = stringify!($name);
            const VERSION: &'static str = "1.0.0";
            const RECORD_TYPE: &'static str = "CombatRecord";
        }
    };
}
rig!(ApplyDamage, "apply-damage");
rig!(ApplyHealing, "apply-healing");
rig!(ApplyStatusEffect, "apply-status-effect");
rig!(RemoveStatusEffect, "remove-status-effect");
rig!(CalculateCooldown, "calculate-cooldown");

pub fn apply_damage(input: CombatInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Combat(combat("apply-damage", &input)),
        ProtocolRecord::Entity(entity("health-damaged", &input)),
    ]
}
pub fn apply_healing(input: CombatInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Combat(combat("apply-healing", &input)),
        ProtocolRecord::Entity(entity("health-healed", &input)),
    ]
}
pub fn apply_status_effect(input: CombatInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Combat(combat("apply-status-effect", &input)),
        ProtocolRecord::Entity(entity("status-applied", &input)),
    ]
}
pub fn remove_status_effect(input: CombatInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Combat(combat("remove-status-effect", &input)),
        ProtocolRecord::Entity(entity("status-removed", &input)),
    ]
}
pub fn calculate_cooldown(input: CombatInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Combat(combat("calculate-cooldown", &input)),
        ProtocolRecord::Entity(entity("cooldown-calculated", &input)),
    ]
}

pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "ApplyDamage",
        "ApplyHealing",
        "ApplyStatusEffect",
        "RemoveStatusEffect",
        "CalculateCooldown",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "CombatRecord,EntityRecord"))
    .collect()
}
