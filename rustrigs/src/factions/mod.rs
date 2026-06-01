use crate::RustrigDescriptor;
use contract_api::protocol_records::{
    fields, EconomyRecord, EntityRecord, ProtocolRecord, WorldRecord,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactionInput {
    pub faction: String,
    pub actor: String,
    pub target: String,
    pub value: i64,
    pub relation: String,
    pub tick: u64,
}
fn w(action: &str, i: &FactionInput) -> WorldRecord {
    WorldRecord::new(
        action,
        i.faction.clone(),
        fields(&[
            ("actor", i.actor.clone()),
            ("target", i.target.clone()),
            ("value", i.value.to_string()),
            ("relation", i.relation.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn ent(action: &str, i: &FactionInput) -> EntityRecord {
    EntityRecord::new(
        action,
        i.actor.clone(),
        fields(&[
            ("faction", i.faction.clone()),
            ("target", i.target.clone()),
            ("value", i.value.to_string()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn econ(action: &str, i: &FactionInput) -> EconomyRecord {
    EconomyRecord::new(
        action,
        i.faction.clone(),
        fields(&[
            ("actor", i.actor.clone()),
            ("value", i.value.to_string()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
pub fn create_faction(i: FactionInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("create-faction", &i)),
        ProtocolRecord::Economy(econ("faction-ledger-opened", &i)),
    ]
}
pub fn assign_member(i: FactionInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("assign-member", &i)),
        ProtocolRecord::Entity(ent("faction-member-assigned", &i)),
    ]
}
pub fn change_reputation(i: FactionInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Entity(ent("change-reputation", &i)),
        ProtocolRecord::Economy(econ("reputation-ledger", &i)),
    ]
}
pub fn declare_relation(i: FactionInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::World(w("declare-relation", &i)),
        ProtocolRecord::Entity(ent("relation-declared", &i)),
    ]
}
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "CreateFaction",
        "AssignMember",
        "ChangeReputation",
        "DeclareRelation",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "WorldRecord,EntityRecord,EconomyRecord"))
    .collect()
}
