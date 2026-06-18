//! Minimal canonicalization kernel for ArenaState commitments.
//!
//! This crate intentionally has no runtime, networking, persistence, journal,
//! federation, deployment, or HotPocket dependencies. It converts an
//! [`ArenaState`] into canonical UTF-8 JSON bytes and hashes those bytes.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationError {
    field: &'static str,
    identifier: String,
}

impl ValidationError {
    pub fn field(&self) -> &'static str {
        self.field
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GAP-2 VALIDATION: FAIL duplicate {} {}",
            self.field, self.identifier
        )
    }
}

impl std::error::Error for ValidationError {}

/// Validate an [`ArenaState`] before canonicalization.
pub fn validate_arena_state(state: &ArenaState) -> Result<(), ValidationError> {
    reject_duplicate_ids(
        "player_id",
        state.players.iter().map(|player| player.player_id.as_str()),
    )?;
    reject_duplicate_ids(
        "entity_id",
        state
            .entities
            .iter()
            .map(|entity| entity.entity_id.as_str()),
    )?;
    reject_duplicate_ids(
        "position.entity_id",
        state
            .positions
            .iter()
            .map(|position| position.entity_id.as_str()),
    )?;
    reject_duplicate_ids(
        "health.entity_id",
        state.health.iter().map(|health| health.entity_id.as_str()),
    )?;
    Ok(())
}

/// Return canonical UTF-8 JSON bytes for an [`ArenaState`] after validation.
pub fn try_canonicalize(state: &ArenaState) -> Result<Vec<u8>, ValidationError> {
    validate_arena_state(state)?;
    Ok(canonicalize_unchecked(state))
}

/// Return canonical UTF-8 JSON bytes for an [`ArenaState`].
pub fn canonicalize(state: &ArenaState) -> Vec<u8> {
    try_canonicalize(state).unwrap_or_else(|err| panic!("{err}"))
}

fn canonicalize_unchecked(state: &ArenaState) -> Vec<u8> {
    let mut canonical = state.clone();
    for player in &mut canonical.players {
        canonicalize_map_values(&mut player.metadata);
    }
    for entity in &mut canonical.entities {
        canonicalize_map_values(&mut entity.attributes);
    }
    canonicalize_map_values(&mut canonical.metadata.extensions);

    canonical
        .players
        .sort_by(|a, b| a.player_id.as_bytes().cmp(b.player_id.as_bytes()));
    canonical
        .entities
        .sort_by(|a, b| a.entity_id.as_bytes().cmp(b.entity_id.as_bytes()));
    canonical
        .positions
        .sort_by(|a, b| a.entity_id.as_bytes().cmp(b.entity_id.as_bytes()));
    canonical
        .health
        .sort_by(|a, b| a.entity_id.as_bytes().cmp(b.entity_id.as_bytes()));
    canonical
        .metadata
        .labels
        .sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));

    serde_json::to_vec(&canonical).expect("canonical JSON serialization is infallible")
}

/// Return the SHA-256 state root as lowercase hex over [`canonicalize`] bytes.
pub fn state_root(state: &ArenaState) -> String {
    hash_bytes(&canonicalize(state))
}

/// Return the SHA-256 world hash as lowercase hex over the ordered roots.
pub fn world_hash(state_root: &str, receipt_root: &str, continuity_root: &str) -> String {
    let state_root = decode_root_hex(state_root, "state_root");
    let receipt_root = decode_root_hex(receipt_root, "receipt_root");
    let continuity_root = decode_root_hex(continuity_root, "continuity_root");
    let mut bytes = Vec::with_capacity(96);
    bytes.extend_from_slice(&state_root);
    bytes.extend_from_slice(&receipt_root);
    bytes.extend_from_slice(&continuity_root);
    hash_bytes(&bytes)
}

fn reject_duplicate_ids<'a>(
    field: &'static str,
    ids: impl Iterator<Item = &'a str>,
) -> Result<(), ValidationError> {
    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(id) {
            return Err(ValidationError {
                field,
                identifier: id.to_string(),
            });
        }
    }
    Ok(())
}

fn decode_root_hex(root: &str, label: &str) -> [u8; 32] {
    let decoded =
        hex::decode(root).unwrap_or_else(|err| panic!("{label} must be lowercase hex: {err}"));
    decoded
        .try_into()
        .unwrap_or_else(|_| panic!("{label} must decode to exactly 32 bytes"))
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
