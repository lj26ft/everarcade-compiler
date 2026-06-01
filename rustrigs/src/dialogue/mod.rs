use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, DialogueRecord, ProtocolRecord, QuestRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialogueInput {
    pub actor: String,
    pub conversation: String,
    pub node: String,
    pub choice: String,
    pub tick: u64,
}
fn d(action: &str, i: &DialogueInput) -> DialogueRecord {
    DialogueRecord::new(
        action,
        i.conversation.clone(),
        fields(&[
            ("actor", i.actor.clone()),
            ("node", i.node.clone()),
            ("choice", i.choice.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn q(action: &str, i: &DialogueInput) -> QuestRecord {
    QuestRecord::new(
        action,
        i.conversation.clone(),
        fields(&[
            ("player", i.actor.clone()),
            ("node", i.node.clone()),
            ("choice", i.choice.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
macro_rules! rig {
    ($name:ident,$action:literal) => {
        pub struct $name;
        impl Rustrig for $name {
            type Input = DialogueInput;
            type Output = DialogueRecord;
            fn execute(input: Self::Input) -> Self::Output {
                d($action, &input)
            }
        }
        impl ReplaySafeRustrig for $name {}
        impl VersionedRustrig for $name {
            const NAME: &'static str = stringify!($name);
            const VERSION: &'static str = "1.0.0";
            const RECORD_TYPE: &'static str = "DialogueRecord";
        }
    };
}
rig!(StartDialogue, "start-dialogue");
rig!(SelectChoice, "select-choice");
rig!(AdvanceNode, "advance-node");
rig!(CompleteDialogue, "complete-dialogue");
pub fn start_dialogue(i: DialogueInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::Dialogue(d("start-dialogue", &i))]
}
pub fn select_choice(i: DialogueInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Dialogue(d("select-choice", &i)),
        ProtocolRecord::Quest(q("dialogue-choice", &i)),
    ]
}
pub fn advance_node(i: DialogueInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::Dialogue(d("advance-node", &i))]
}
pub fn complete_dialogue(i: DialogueInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Dialogue(d("complete-dialogue", &i)),
        ProtocolRecord::Quest(q("dialogue-complete", &i)),
    ]
}
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "StartDialogue",
        "SelectChoice",
        "AdvanceNode",
        "CompleteDialogue",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "DialogueRecord,QuestRecord"))
    .collect()
}
