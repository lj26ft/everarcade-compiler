use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{
    fields, EconomyRecord, InventoryRecord, ProtocolRecord, QuestRecord,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuestInput {
    pub player: String,
    pub quest: String,
    pub step: String,
    pub reward: String,
    pub tick: u64,
}
fn q(action: &str, i: &QuestInput) -> QuestRecord {
    QuestRecord::new(
        action,
        i.quest.clone(),
        fields(&[
            ("player", i.player.clone()),
            ("step", i.step.clone()),
            ("reward", i.reward.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn inv(action: &str, i: &QuestInput) -> InventoryRecord {
    InventoryRecord::new(
        action,
        i.player.clone(),
        fields(&[
            ("quest", i.quest.clone()),
            ("reward", i.reward.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn econ(action: &str, i: &QuestInput) -> EconomyRecord {
    EconomyRecord::new(
        action,
        i.player.clone(),
        fields(&[
            ("quest", i.quest.clone()),
            ("reward", i.reward.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
macro_rules! rig {
    ($name:ident,$action:literal) => {
        pub struct $name;
        impl Rustrig for $name {
            type Input = QuestInput;
            type Output = QuestRecord;
            fn execute(input: Self::Input) -> Self::Output {
                q($action, &input)
            }
        }
        impl ReplaySafeRustrig for $name {}
        impl VersionedRustrig for $name {
            const NAME: &'static str = stringify!($name);
            const VERSION: &'static str = "1.0.0";
            const RECORD_TYPE: &'static str = "QuestRecord";
        }
    };
}
rig!(StartQuest, "start-quest");
rig!(AdvanceQuest, "advance-quest");
rig!(AdvanceObjective, "advance-objective");
rig!(CompleteQuest, "complete-quest");
rig!(FailQuest, "fail-quest");
rig!(GrantReward, "grant-reward");
pub fn start_quest(i: QuestInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::Quest(q("start-quest", &i))]
}
pub fn advance_objective(i: QuestInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::Quest(q("advance-objective", &i))]
}
pub fn complete_quest(i: QuestInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Quest(q("complete-quest", &i)),
        ProtocolRecord::Inventory(inv("quest-reward-item", &i)),
        ProtocolRecord::Economy(econ("quest-reward-ledger", &i)),
    ]
}
pub fn fail_quest(i: QuestInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::Quest(q("fail-quest", &i))]
}
pub fn grant_reward(i: QuestInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Quest(q("grant-reward", &i)),
        ProtocolRecord::Inventory(inv("grant-reward-item", &i)),
        ProtocolRecord::Economy(econ("grant-reward-ledger", &i)),
    ]
}
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "StartQuest",
        "AdvanceObjective",
        "CompleteQuest",
        "FailQuest",
        "GrantReward",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "QuestRecord,InventoryRecord,EconomyRecord"))
    .collect()
}
