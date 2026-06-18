use canonicalizer_kernel::{
    canonicalize, state_root, validate_arena_state, world_hash, ArenaState,
};
use std::fs;
use std::path::PathBuf;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("proofs/handoff-v1/canonical-fixtures")
}

#[test]
fn handoff_v1_fixtures_match_canonical_bytes_and_roots() {
    let dir = fixture_dir();
    for id in ["001", "002", "003", "004", "005"] {
        let state_json = fs::read_to_string(dir.join(format!("fixture-{id}-state.json"))).unwrap();
        let expected_hex =
            fs::read_to_string(dir.join(format!("fixture-{id}-canonical.hex"))).unwrap();
        let expected_root = fs::read_to_string(dir.join(format!("fixture-{id}-root.txt"))).unwrap();
        let state: ArenaState = serde_json::from_str(&state_json).unwrap();

        assert_eq!(
            hex::encode(canonicalize(&state)),
            expected_hex.trim(),
            "canonical bytes for fixture-{id}"
        );
        assert_eq!(
            state_root(&state),
            expected_root.trim(),
            "state root for fixture-{id}"
        );
    }
}

#[test]
fn world_hash_is_ordered_sha256_over_root_bytes() {
    let hash = world_hash(
        "0233d36e2e73ed0a123dc4d75cb9dd9792cdbc0e7ea078309bd01255375468a8",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
    );
    assert_eq!(
        hash,
        "5284cf69397a12b8453ebbb82795dfd01641d503f4967a245c711bfc43b65407"
    );
}

fn differential_base_state() -> serde_json::Value {
    serde_json::json!({
      "schema_version": 1,
      "world_id": "w",
      "arena_id": "a",
      "tick": 0,
      "players": [],
      "entities": [],
      "positions": [],
      "health": [],
      "receipts": { "receipt_root": "0000000000000000000000000000000000000000000000000000000000000000", "receipt_count": 0, "last_receipt_hash": null },
      "continuity": { "continuity_root": "0000000000000000000000000000000000000000000000000000000000000000", "previous_state_root": null, "replay_root": "0000000000000000000000000000000000000000000000000000000000000000", "migration_root": null, "epoch": 0 },
      "metadata": { "ruleset_id": "r", "ruleset_version": 1, "created_by": null, "labels": [], "extensions": {} }
    })
}

#[test]
fn differential_vec_1_sorts_players_before_state_root() {
    let mut value = differential_base_state();
    value["players"] = serde_json::json!([
        {"player_id":"p2","controller_id":"c","join_tick":0,"status":"active","score":0,"metadata":{}},
        {"player_id":"p1","controller_id":"c","join_tick":0,"status":"active","score":0,"metadata":{}}
    ]);
    let state: ArenaState = serde_json::from_value(value).unwrap();
    let canonical = String::from_utf8(canonicalize(&state)).unwrap();
    assert!(
        canonical.find(r#""player_id":"p1""#).unwrap()
            < canonical.find(r#""player_id":"p2""#).unwrap()
    );
    assert_eq!(
        state_root(&state),
        "36b0092d3c19761944556d66bef716b2fdfbd253915ac5c3ffdeec898077fc00"
    );
}

#[test]
fn differential_vec_2_sorts_metadata_labels_before_state_root() {
    let mut value = differential_base_state();
    value["metadata"]["labels"] = serde_json::json!(["b", "a"]);
    let state: ArenaState = serde_json::from_value(value).unwrap();
    assert!(String::from_utf8(canonicalize(&state))
        .unwrap()
        .contains(r#""labels":["a","b"]"#));
    assert_eq!(
        state_root(&state),
        "8a3612449a2a00d9a84ff9d19f84e839b9d1779b4d86a6f77f9dfa98fb989b42"
    );
}

#[test]
fn differential_vec_3_world_hash_uses_decoded_root_bytes() {
    let hash = world_hash(
        "0233d36e2e73ed0a123dc4d75cb9dd9792cdbc0e7ea078309bd01255375468a8",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
    );
    assert_ne!(
        hash,
        "cf08731c589cdcc75bce9451c68663ce8c3afbec9159adf2912477a6814d4e4d"
    );
    assert_eq!(
        hash,
        "5284cf69397a12b8453ebbb82795dfd01641d503f4967a245c711bfc43b65407"
    );
}

#[test]
fn gap_2_duplicate_identifier_fixtures_are_rejected_before_canonicalization() {
    let cases = [
        (
            "fixture-gap2-duplicate-player-state.json",
            "player_id",
            "ERROR duplicate player_id: player-dup",
        ),
        (
            "fixture-gap2-duplicate-entity-state.json",
            "entity_id",
            "ERROR duplicate entity_id: entity-dup",
        ),
        (
            "fixture-gap2-duplicate-position-state.json",
            "position.entity_id",
            "ERROR duplicate position entity_id: entity-dup",
        ),
        (
            "fixture-gap2-duplicate-health-state.json",
            "health.entity_id",
            "ERROR duplicate health entity_id: entity-dup",
        ),
    ];

    for (fixture, field, expected_error) in cases {
        let state_json = fs::read_to_string(fixture_dir().join(fixture)).unwrap();
        let state: ArenaState = serde_json::from_str(&state_json).unwrap();
        let error = validate_arena_state(&state).unwrap_err();

        assert_eq!(error.field(), field, "field for {fixture}");
        assert_eq!(error.to_string(), expected_error, "error for {fixture}");

        let panic = std::panic::catch_unwind(|| canonicalize(&state)).unwrap_err();
        let panic_message = panic
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| panic.downcast_ref::<&str>().copied())
            .unwrap();
        assert_eq!(panic_message, expected_error, "panic for {fixture}");
    }
}

#[test]
fn gap_2_validation_passes_unique_fixture_set() {
    let dir = fixture_dir();
    for id in ["001", "002", "003", "004", "005"] {
        let state_json = fs::read_to_string(dir.join(format!("fixture-{id}-state.json"))).unwrap();
        let state: ArenaState = serde_json::from_str(&state_json).unwrap();
        validate_arena_state(&state).unwrap();
    }
    println!("GAP-2 VALIDATION: PASS");
}
