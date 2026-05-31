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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnchorRecord {
    pub record_type: &'static str,
    pub id: String,
    pub payload_hash: String,
    pub anchor_hash: String,
}

pub type ReceiptAnchorRecord = AnchorRecord;
pub type ReplayAnchorRecord = AnchorRecord;
pub type WorldAnchorRecord = AnchorRecord;
pub type DeploymentAnchorRecord = AnchorRecord;

fn stable_anchor_hash(record_type: &str, id: &str, payload_hash: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for field in [record_type, id, payload_hash] {
        for byte in field.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("anchor:{record_type}:{hash:016x}")
}

pub fn create_anchor_record(
    record_type: &'static str,
    id: impl Into<String>,
    payload_hash: impl Into<String>,
) -> AnchorRecord {
    let id = id.into();
    let payload_hash = payload_hash.into();
    let anchor_hash = stable_anchor_hash(record_type, &id, &payload_hash);
    AnchorRecord {
        record_type,
        id,
        payload_hash,
        anchor_hash,
    }
}

pub fn arena_vanguard_anchor_records() -> Vec<AnchorRecord> {
    vec![
        create_anchor_record(
            "ReceiptAnchorRecord",
            "arena-vanguard-receipt-0003",
            "receipt:arena-vanguard:tick:3:world:players:2",
        ),
        create_anchor_record(
            "ReplayAnchorRecord",
            "arena-vanguard-replay-0003",
            "replay:arena-vanguard:events:4",
        ),
        create_anchor_record(
            "WorldAnchorRecord",
            "arena-vanguard-world-0003",
            "world:arena-vanguard:tick:3:players:2:marketplace:6",
        ),
        create_anchor_record(
            "DeploymentAnchorRecord",
            "arena-vanguard-deployment-0001",
            "deployment:evernode:arena-vanguard:ops:6",
        ),
    ]
}

pub fn verify_anchor_record(record: &AnchorRecord) -> bool {
    record.anchor_hash == stable_anchor_hash(record.record_type, &record.id, &record.payload_hash)
}

pub fn anchor_records_equivalent(first: &[AnchorRecord], second: &[AnchorRecord]) -> bool {
    first == second && first.iter().all(verify_anchor_record)
}
