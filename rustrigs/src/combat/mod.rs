use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, CombatRecord};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombatInput {
    pub actor: String,
    pub target: String,
    pub amount: u64,
    pub status: String,
    pub tick: u64,
}
fn rec(action: &str, i: CombatInput) -> CombatRecord {
    CombatRecord::new(
        action,
        i.target,
        fields(&[
            ("actor", i.actor),
            ("amount", i.amount.to_string()),
            ("status", i.status),
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
                rec($action, input)
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
rig!(CalculateThreat, "calculate-threat");
rig!(CalculateCooldown, "calculate-cooldown");
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "ApplyDamage",
        "ApplyHealing",
        "ApplyStatusEffect",
        "RemoveStatusEffect",
        "CalculateThreat",
        "CalculateCooldown",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "CombatRecord"))
    .collect()
}
