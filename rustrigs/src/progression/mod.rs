use crate::RustrigDescriptor;
use contract_api::protocol_records::{
    fields, EconomyRecord, EntityRecord, ProtocolRecord, QuestRecord,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgressionInput {
    pub actor: String,
    pub track: String,
    pub amount: u64,
    pub milestone: String,
    pub tick: u64,
}
fn ent(action: &str, i: &ProgressionInput) -> EntityRecord {
    EntityRecord::new(
        action,
        i.actor.clone(),
        fields(&[
            ("track", i.track.clone()),
            ("amount", i.amount.to_string()),
            ("milestone", i.milestone.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn q(action: &str, i: &ProgressionInput) -> QuestRecord {
    QuestRecord::new(
        action,
        i.track.clone(),
        fields(&[
            ("actor", i.actor.clone()),
            ("amount", i.amount.to_string()),
            ("milestone", i.milestone.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn econ(action: &str, i: &ProgressionInput) -> EconomyRecord {
    EconomyRecord::new(
        action,
        i.actor.clone(),
        fields(&[
            ("track", i.track.clone()),
            ("amount", i.amount.to_string()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
pub fn grant_experience(i: ProgressionInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Entity(ent("grant-experience", &i)),
        ProtocolRecord::Economy(econ("xp-ledger", &i)),
    ]
}
pub fn unlock_milestone(i: ProgressionInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Entity(ent("unlock-milestone", &i)),
        ProtocolRecord::Quest(q("milestone-unlocked", &i)),
    ]
}
pub fn descriptors() -> Vec<RustrigDescriptor> {
    ["GrantExperience", "UnlockMilestone"]
        .into_iter()
        .map(|n| RustrigDescriptor::new(n, "1.0.0", "EntityRecord,QuestRecord,EconomyRecord"))
        .collect()
}
