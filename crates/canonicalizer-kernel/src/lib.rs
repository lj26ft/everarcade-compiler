//! Minimal canonicalization kernel for ArenaState commitments.
//!
//! This crate intentionally has no runtime, networking, persistence, journal,
//! federation, deployment, or HotPocket dependencies. It converts an
//! [`ArenaState`] into canonical UTF-8 JSON bytes and hashes those bytes.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Arena state accepted by the standalone canonicalizer kernel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArenaState {
    pub schema_version: u64,
    pub world_id: String,
    pub arena_id: String,
    pub tick: u64,
    pub players: Vec<Player>,
    pub entities: Vec<Entity>,
    pub positions: Vec<Position>,
    pub health: Vec<Health>,
    pub receipts: ReceiptState,
    pub continuity: ContinuityState,
    pub metadata: ArenaMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub player_id: String,
    pub controller_id: String,
    pub join_tick: u64,
    pub status: String,
    pub score: i64,
    pub metadata: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entity {
    pub entity_id: String,
    pub entity_type: String,
    pub owner_player_id: Option<String>,
    pub spawn_tick: u64,
    pub despawn_tick: Option<u64>,
    pub attributes: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub entity_id: String,
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub rotation: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Health {
    pub entity_id: String,
    pub current: i64,
    pub maximum: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReceiptState {
    pub receipt_root: String,
    pub receipt_count: u64,
    pub last_receipt_hash: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityState {
    pub continuity_root: String,
    pub previous_state_root: Option<String>,
    pub replay_root: String,
    pub migration_root: Option<String>,
    pub epoch: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArenaMetadata {
    pub ruleset_id: String,
    pub ruleset_version: u64,
    pub created_by: Option<String>,
    pub labels: Vec<String>,
    pub extensions: BTreeMap<String, Value>,
}

/// Return canonical UTF-8 JSON bytes for an [`ArenaState`].
pub fn canonicalize(state: &ArenaState) -> Vec<u8> {
    let mut canonical = state.clone();
    for player in &mut canonical.players {
        canonicalize_map_values(&mut player.metadata);
    }
    for entity in &mut canonical.entities {
        canonicalize_map_values(&mut entity.attributes);
    }
    canonicalize_map_values(&mut canonical.metadata.extensions);

    serde_json::to_vec(&canonical).expect("canonical JSON serialization is infallible")
}

/// Return the SHA-256 state root as lowercase hex over [`canonicalize`] bytes.
pub fn state_root(state: &ArenaState) -> String {
    hash_bytes(&canonicalize(state))
}

/// Return the SHA-256 world hash as lowercase hex over the ordered roots.
pub fn world_hash(state_root: &str, receipt_root: &str, continuity_root: &str) -> String {
    let mut bytes =
        Vec::with_capacity(state_root.len() + receipt_root.len() + continuity_root.len());
    bytes.extend_from_slice(state_root.as_bytes());
    bytes.extend_from_slice(receipt_root.as_bytes());
    bytes.extend_from_slice(continuity_root.as_bytes());
    hash_bytes(&bytes)
}

fn hash_bytes(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn canonicalize_map_values(map: &mut BTreeMap<String, Value>) {
    for value in map.values_mut() {
        *value = canonical_value(std::mem::take(value));
    }
}

fn canonical_value(value: Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.into_iter().map(canonical_value).collect()),
        Value::Object(map) => {
            let ordered: BTreeMap<String, Value> = map
                .into_iter()
                .map(|(key, value)| (key, canonical_value(value)))
                .collect();
            let mut sorted = Map::new();
            for (key, value) in ordered {
                sorted.insert(key, value);
            }
            Value::Object(sorted)
        }
        scalar => scalar,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_key_order_uses_bytes() {
        let mut map = Map::new();
        map.insert("𐀀".to_string(), Value::from(5));
        map.insert("".to_string(), Value::from(4));
        map.insert("é".to_string(), Value::from(3));
        map.insert("a".to_string(), Value::from(2));
        map.insert("A".to_string(), Value::from(1));

        let bytes = serde_json::to_vec(&canonical_value(Value::Object(map))).unwrap();
        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            r#"{"A":1,"a":2,"é":3,"":4,"𐀀":5}"#
        );
    }
}
