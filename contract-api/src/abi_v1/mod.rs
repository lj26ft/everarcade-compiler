use crate::protocol_records::ProtocolRecord;
use serde::Serialize;
use std::collections::BTreeMap;

pub const ABI_VERSION: u16 = 1;
pub const ABI_FREEZE_POLICY: &str = "append-only backward-compatible deterministic replay-safe";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiV1Readiness {
    pub append_only: bool,
    pub backward_compatible: bool,
    pub deterministic_serialization: bool,
    pub canonical_hashing: bool,
    pub replay_safe: bool,
}

pub fn readiness() -> AbiV1Readiness {
    AbiV1Readiness {
        append_only: true,
        backward_compatible: true,
        deterministic_serialization: true,
        canonical_hashing: true,
        replay_safe: true,
    }
}

pub fn canonical_bytes<T: Serialize>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).expect("ABI v1 records must serialize deterministically")
}

pub fn canonical_hash<T: Serialize>(value: &T) -> String {
    let bytes = canonical_bytes(value);
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub fn record_hash(record: &ProtocolRecord) -> String {
    canonical_hash(record)
}

pub fn manifest() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("world", "WorldRecord"),
        ("entity", "EntityRecord"),
        ("economy", "EconomyRecord"),
        ("inventory", "InventoryRecord"),
        ("quest", "QuestRecord"),
        ("dialogue", "DialogueRecord"),
        ("combat", "CombatRecord"),
        ("ui", "UiRecord"),
        ("replay", "ReplayRecord"),
        ("deployment", "DeploymentRecord|DeploymentIntentRecord"),
        ("xrpl", "XrplRecord|XrplIntentRecord"),
    ])
}
