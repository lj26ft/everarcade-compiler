use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, DialogueRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialogueInput {
    pub actor: String,
    pub conversation: String,
    pub node: String,
    pub choice: String,
    pub tick: u64,
}
fn rec(action: &str, i: DialogueInput) -> DialogueRecord {
    DialogueRecord::new(
        action,
        i.conversation,
        fields(&[
            ("actor", i.actor),
            ("node", i.node),
            ("choice", i.choice),
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
                rec($action, input)
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
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "StartDialogue",
        "SelectChoice",
        "AdvanceNode",
        "CompleteDialogue",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "DialogueRecord"))
    .collect()
}
