use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, XrplIntentRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XrplIntentInput {
    pub account: String,
    pub intent: String,
    pub asset: String,
    pub amount: u64,
    pub destination: String,
    pub tick: u64,
}
pub struct CreateXrplIntent;
impl Rustrig for CreateXrplIntent {
    type Input = XrplIntentInput;
    type Output = XrplIntentRecord;
    fn execute(i: Self::Input) -> Self::Output {
        XrplIntentRecord::new(
            "create-xrpl-intent",
            i.account,
            fields(&[
                ("intent", i.intent),
                ("asset", i.asset),
                ("amount", i.amount.to_string()),
                ("destination", i.destination),
                ("tick", i.tick.to_string()),
                ("submission", String::from("runtime-bridge-only")),
            ]),
        )
    }
}
impl ReplaySafeRustrig for CreateXrplIntent {}
impl VersionedRustrig for CreateXrplIntent {
    const NAME: &'static str = "CreateXrplIntent";
    const VERSION: &'static str = "1.0.0";
    const RECORD_TYPE: &'static str = "XrplIntentRecord";
}
pub fn descriptors() -> Vec<RustrigDescriptor> {
    vec![RustrigDescriptor::new(
        "CreateXrplIntent",
        "1.0.0",
        "XrplIntentRecord",
    )]
}
