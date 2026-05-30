use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, QuestRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuestInput {
    pub player: String,
    pub quest: String,
    pub step: String,
    pub reward: String,
    pub tick: u64,
}
fn rec(action: &str, i: QuestInput) -> QuestRecord {
    QuestRecord::new(
        action,
        i.quest,
        fields(&[
            ("player", i.player),
            ("step", i.step),
            ("reward", i.reward),
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
                rec($action, input)
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
rig!(CompleteQuest, "complete-quest");
rig!(FailQuest, "fail-quest");
rig!(GrantReward, "grant-reward");
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "StartQuest",
        "AdvanceQuest",
        "CompleteQuest",
        "FailQuest",
        "GrantReward",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "QuestRecord"))
    .collect()
}
