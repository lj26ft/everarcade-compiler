use canonicalizer_kernel::{canonicalize, state_root, world_hash, ArenaState};
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
    for id in ["001", "002", "003"] {
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
    let hash = world_hash("state", "receipt", "continuity");
    assert_eq!(
        hash,
        "da37679e3e7ddfdaddca0f36b921c35040b6052585a8cee0b4c40cb00c21c8d2"
    );
}
