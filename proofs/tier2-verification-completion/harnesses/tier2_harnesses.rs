use everarcade_tier2_proof::{commitment, continuity_root, require_value, world_hash};

fn assert_root(fixture: &str, key: &str, parts: &[&str]) -> String {
    let expected = require_value(fixture, key);
    let actual = commitment(parts);
    assert_eq!(actual, expected, "{key}");
    actual
}

#[test]
fn replay_harness() {
    let fx = include_str!("../fixtures/replay.fixture");
    let state = assert_root(
        fx,
        "expected_state_root",
        &[
            "replay-state",
            &require_value(fx, "inputs"),
            &require_value(fx, "journal"),
        ],
    );
    let receipt = assert_root(
        fx,
        "expected_receipt_root",
        &["replay-receipts", &require_value(fx, "journal")],
    );
    assert_eq!(
        world_hash(
            &state,
            &receipt,
            &require_value(fx, "expected_continuity_root")
        ),
        require_value(fx, "expected_world_hash")
    );
    assert_eq!(
        continuity_root(&require_value(fx, "genesis_continuity_root"), &state),
        require_value(fx, "expected_continuity_root")
    );
    println!("REPLAY VERIFIED");
}

#[test]
fn restore_harness() {
    let fx = include_str!("../fixtures/restore.fixture");
    let state = assert_root(
        fx,
        "expected_state_root",
        &[
            "restore-state",
            &require_value(fx, "checkpoint"),
            &require_value(fx, "restore_input"),
        ],
    );
    assert_eq!(
        continuity_root(&require_value(fx, "checkpoint_continuity_root"), &state),
        require_value(fx, "expected_continuity_root")
    );
    println!("RESTORE VERIFIED");
}

#[test]
fn migration_harness() {
    let fx = include_str!("../fixtures/migration.fixture");
    let source = assert_root(
        fx,
        "expected_source_root",
        &["source-world", &require_value(fx, "source_world")],
    );
    let destination = assert_root(
        fx,
        "expected_destination_root",
        &[
            "destination-world",
            &require_value(fx, "destination_world"),
            &require_value(fx, "migration_inputs"),
        ],
    );
    assert_eq!(source, require_value(fx, "preserved_source_root"));
    assert_eq!(
        continuity_root(&source, &destination),
        require_value(fx, "expected_continuity_root")
    );
    println!("MIGRATION VERIFIED");
}

#[test]
fn federation_harness() {
    let fx = include_str!("../fixtures/federation.fixture");
    let commitment_root = assert_root(
        fx,
        "expected_commitment",
        &[
            "federation-op",
            &require_value(fx, "world_a"),
            &require_value(fx, "world_b"),
            &require_value(fx, "cross_world_operation"),
        ],
    );
    assert_eq!(
        commitment(&["federation-root", &commitment_root]),
        require_value(fx, "expected_root")
    );
    println!("FEDERATION VERIFIED");
}

#[test]
fn js_equivalence_harness() {
    let fx = include_str!("../fixtures/js-equivalence.fixture");
    let js = assert_root(
        fx,
        "js_output",
        &["js-kernel", &require_value(fx, "shared_fixture_set")],
    );
    let rust = assert_root(
        fx,
        "rust_output",
        &["js-kernel", &require_value(fx, "shared_fixture_set")],
    );
    assert_eq!(js, rust);
    assert_eq!(js, require_value(fx, "expected_roots"));
    println!("JS KERNEL EQUIVALENCE VERIFIED");
}
