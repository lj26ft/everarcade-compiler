use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type RecordFields = BTreeMap<String, String>;

macro_rules! define_record {
    ($name:ident, $kind:literal) => {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            pub version: u16,
            pub action: String,
            pub subject: String,
            pub fields: RecordFields,
        }

        impl $name {
            pub const CATEGORY: &'static str = $kind;

            pub fn new(
                action: impl Into<String>,
                subject: impl Into<String>,
                fields: RecordFields,
            ) -> Self {
                Self {
                    version: 1,
                    action: action.into(),
                    subject: subject.into(),
                    fields,
                }
            }
        }
    };
}

define_record!(WorldRecord, "world");
define_record!(EntityRecord, "entity");
define_record!(EconomyRecord, "economy");
define_record!(InventoryRecord, "inventory");
define_record!(QuestRecord, "quest");
define_record!(DialogueRecord, "dialogue");
define_record!(CombatRecord, "combat");
define_record!(UiRecord, "ui");
define_record!(ReplayRecord, "replay");
define_record!(DeploymentRecord, "deployment");
define_record!(DeploymentIntentRecord, "deployment-intent");
define_record!(XrplRecord, "xrpl");
define_record!(XrplIntentRecord, "xrpl-intent");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolRecord {
    World(WorldRecord),
    Entity(EntityRecord),
    Economy(EconomyRecord),
    Inventory(InventoryRecord),
    Quest(QuestRecord),
    Dialogue(DialogueRecord),
    Combat(CombatRecord),
    Ui(UiRecord),
    Replay(ReplayRecord),
    Deployment(DeploymentRecord),
    DeploymentIntent(DeploymentIntentRecord),
    Xrpl(XrplRecord),
    XrplIntent(XrplIntentRecord),
}

pub fn fields(entries: &[(&str, String)]) -> RecordFields {
    entries
        .iter()
        .map(|(key, value)| ((*key).to_owned(), value.clone()))
        .collect()
}
