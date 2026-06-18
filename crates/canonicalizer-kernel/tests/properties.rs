//! Differential property proof for the bounded Tier-2 (Root Integrity) session.
//!
//! These harnesses prove, over thousands of randomized type-valid `ArenaState`
//! values, the three protocol invariants the canonicalization boundary rests on:
//!
//!   * Determinism  : `canonicalize(s) == canonicalize(s)`
//!   * Order-invariance : permuting unordered collections / dynamic-map key
//!     insertion order does not change canonical bytes (unique sort keys).
//!   * Root integrity (Property 2): `state_root(s) == hex(SHA256(canonicalize(s)))`
//!     with no prefix, salt, or trailing newline.
//!
//! The independent cross-implementation oracle (Python) lives in
//! `tools/differential.py`; this file is the in-process Rust side.

use canonicalizer_kernel::{canonicalize, state_root, ArenaState, ContinuityState};
use proptest::collection::{hash_set, vec};
use proptest::prelude::*;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const ZERO_ROOT: &str = "0000000000000000000000000000000000000000000000000000000000000000";

// Identifiers and keys deliberately include multibyte UTF-8 (é, 𐀀) so the
// byte-lexicographic ordering boundary is exercised, not just ASCII.
fn arb_str() -> impl Strategy<Value = String> {
    "[A-Za-z0-9_é𐀀\\-]{1,6}".prop_map(|s| s)
}

fn arb_status() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("active"),
        Just("inactive"),
        Just("eliminated"),
        Just("left"),
    ]
    .prop_map(String::from)
}

// Arbitrary deterministic-domain JSON value: int / string / bool / null /
// array / object. NO floats (outside the consensus value domain).
fn arb_value() -> impl Strategy<Value = Value> {
    let leaf = prop_oneof![
        Just(Value::Null),
        any::<bool>().prop_map(Value::Bool),
        any::<i64>().prop_map(|n| Value::from(n)),
        arb_str().prop_map(Value::String),
    ];
    leaf.prop_recursive(3, 12, 4, |inner| {
        prop_oneof![
            vec(inner.clone(), 0..4).prop_map(Value::Array),
            // Build objects from a Vec so insertion order is arbitrary
            // (serde_json preserve_order keeps insertion order pre-canon).
            vec((arb_str(), inner), 0..4).prop_map(|kvs| {
                let mut m = Map::new();
                for (k, v) in kvs {
                    m.insert(k, v);
                }
                Value::Object(m)
            }),
        ]
    })
}

fn arb_dynmap() -> impl Strategy<Value = BTreeMap<String, Value>> {
    vec((arb_str(), arb_value()), 0..4).prop_map(|kvs| kvs.into_iter().collect())
}

// A pool of `n` distinct ids (uniqueness needed for order-invariance, since a
// stable sort over duplicate keys is sensitive to input order).
fn unique_ids(max: usize) -> impl Strategy<Value = Vec<String>> {
    hash_set(arb_str(), 0..=max).prop_map(|s| s.into_iter().collect())
}

fn arb_state(unique: bool) -> BoxedStrategy<ArenaState> {
    let ids = if unique {
        unique_ids(5).boxed()
    } else {
        vec(arb_str(), 0..5).boxed()
    };
    let eids = if unique {
        unique_ids(5).boxed()
    } else {
        vec(arb_str(), 0..5).boxed()
    };
    (
        any::<u64>(),
        arb_str(),
        arb_str(),
        any::<u64>(),
        ids,
        eids,
        unique_ids(5),
        unique_ids(5),
        unique_ids(6),
    )
        .prop_flat_map(
            |(schema, world, arena, tick, pids, eids, pos_ids, hp_ids, labels)| {
                let players = pids
                    .into_iter()
                    .map(|id| {
                        (
                            Just(id),
                            arb_str(),
                            any::<u64>(),
                            arb_status(),
                            any::<i64>(),
                            arb_dynmap(),
                        )
                    })
                    .collect::<Vec<_>>();
                let entities = eids
                    .into_iter()
                    .map(|id| {
                        (
                            Just(id),
                            arb_str(),
                            any::<Option<String>>(),
                            any::<u64>(),
                            any::<Option<u64>>(),
                            arb_dynmap(),
                        )
                    })
                    .collect::<Vec<_>>();
                let positions = pos_ids
                    .into_iter()
                    .map(|id| (Just(id), any::<i64>(), any::<i64>(), any::<i64>(), any::<i64>()))
                    .collect::<Vec<_>>();
                let health = hp_ids
                    .into_iter()
                    .map(|id| (Just(id), any::<i64>(), any::<i64>()))
                    .collect::<Vec<_>>();
                (
                    Just((schema, world, arena, tick, labels)),
                    players,
                    entities,
                    positions,
                    health,
                    arb_dynmap(),
                )
            },
        )
        .prop_map(
            |((schema, world, arena, tick, labels), players, entities, positions, health, ext)| {
                use canonicalizer_kernel::{
                    ArenaMetadata, Entity, Health, Player, Position, ReceiptState,
                };
                ArenaState {
                    schema_version: schema,
                    world_id: world,
                    arena_id: arena,
                    tick,
                    players: players
                        .into_iter()
                        .map(|(player_id, controller_id, join_tick, status, score, metadata)| {
                            Player {
                                player_id,
                                controller_id,
                                join_tick,
                                status,
                                score,
                                metadata,
                            }
                        })
                        .collect(),
                    entities: entities
                        .into_iter()
                        .map(
                            |(entity_id, entity_type, owner_player_id, spawn_tick, despawn_tick, attributes)| {
                                Entity {
                                    entity_id,
                                    entity_type,
                                    owner_player_id,
                                    spawn_tick,
                                    despawn_tick,
                                    attributes,
                                }
                            },
                        )
                        .collect(),
                    positions: positions
                        .into_iter()
                        .map(|(entity_id, x, y, z, rotation)| Position {
                            entity_id,
                            x,
                            y,
                            z,
                            rotation,
                        })
                        .collect(),
                    health: health
                        .into_iter()
                        .map(|(entity_id, current, maximum)| Health {
                            entity_id,
                            current,
                            maximum,
                        })
                        .collect(),
                    receipts: ReceiptState {
                        receipt_root: ZERO_ROOT.to_string(),
                        receipt_count: 0,
                        last_receipt_hash: None,
                    },
                    continuity: ContinuityState {
                        continuity_root: ZERO_ROOT.to_string(),
                        previous_state_root: None,
                        replay_root: ZERO_ROOT.to_string(),
                        migration_root: None,
                        epoch: 0,
                    },
                    metadata: ArenaMetadata {
                        ruleset_id: "r".to_string(),
                        ruleset_version: 1,
                        created_by: None,
                        labels,
                        extensions: ext,
                    },
                }
            },
        )
        .boxed()
}

// Reverse the insertion order of every JSON object, recursively. Canonicalization
// must be invariant to this.
fn permute_value(v: &Value) -> Value {
    match v {
        Value::Array(a) => Value::Array(a.iter().map(permute_value).collect()),
        Value::Object(m) => {
            let mut out = Map::new();
            for (k, val) in m.iter().rev() {
                out.insert(k.clone(), permute_value(val));
            }
            Value::Object(out)
        }
        other => other.clone(),
    }
}

fn permute(state: &ArenaState) -> ArenaState {
    let mut s = state.clone();
    s.players.reverse();
    s.entities.reverse();
    s.positions.reverse();
    s.health.reverse();
    s.metadata.labels.reverse();
    for p in &mut s.players {
        p.metadata = p.metadata.iter().map(|(k, v)| (k.clone(), permute_value(v))).collect();
    }
    for e in &mut s.entities {
        e.attributes = e.attributes.iter().map(|(k, v)| (k.clone(), permute_value(v))).collect();
    }
    s.metadata.extensions = s
        .metadata
        .extensions
        .iter()
        .map(|(k, v)| (k.clone(), permute_value(v)))
        .collect();
    s
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 4000, ..ProptestConfig::default() })]

    /// Property 1 (Determinism) + Property 2 (Root Integrity), over arbitrary
    /// type-valid states. Root is exactly SHA256 over canonical bytes — no
    /// prefix, salt, or trailing newline — and canonical bytes are valid UTF-8.
    #[test]
    fn determinism_and_root_integrity(state in arb_state(false)) {
        let a = canonicalize(&state);
        let b = canonicalize(&state);
        prop_assert_eq!(&a, &b, "canonicalize is not deterministic");

        prop_assert!(std::str::from_utf8(&a).is_ok(), "canonical bytes not valid UTF-8");

        let expected = hex::encode(Sha256::digest(&a));
        prop_assert_eq!(state_root(&state), expected, "state_root != SHA256(canonical bytes)");
    }

    /// Order-invariance: permuting unordered collections and dynamic-map key
    /// insertion order yields identical canonical bytes and identical root
    /// (sort keys are unique, so the stable sort is order-independent).
    #[test]
    fn order_invariance(state in arb_state(true)) {
        let permuted = permute(&state);
        prop_assert_eq!(
            canonicalize(&state),
            canonicalize(&permuted),
            "canonical bytes differ under permutation"
        );
        prop_assert_eq!(
            state_root(&state),
            state_root(&permuted),
            "state_root differs under permutation"
        );
    }
}
