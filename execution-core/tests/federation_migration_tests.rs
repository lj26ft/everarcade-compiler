mod common;

use std::{fs, path::PathBuf};

use execution_core::federation::{
    node::FederationNodeId,
    runtime::{freeze_world, migrate_world, resume_world, WorldMigrationRequest},
};

fn fixture(name: &str) -> PathBuf {
    common::fixtures::ensure_repo_counter_world_fixtures();
    common::fixtures::repo_counter_world_fixture_dir().join(name)
}

fn seed_world(root: &std::path::Path) {
    fs::create_dir_all(root.join("package")).unwrap();
    fs::create_dir_all(root.join("receipts")).unwrap();
    fs::copy(fixture("world.wasm"), root.join("package/world.wasm")).unwrap();
    fs::copy(fixture("checkpoint_0.bin"), root.join("checkpoint.bin")).unwrap();
    fs::copy(fixture("lineage.bin"), root.join("lineage.bin")).unwrap();
    fs::copy(fixture("manifest.bin"), root.join("manifest.bin")).unwrap();
    fs::copy(
        fixture("recovery_descriptor.bin"),
        root.join("descriptor.bin"),
    )
    .unwrap();
    fs::copy(
        fixture("receipt_1.bin"),
        root.join("receipts/receipt_0000000000000000.bin"),
    )
    .unwrap();
    fs::copy(
        fixture("receipt_2.bin"),
        root.join("receipts/receipt_0000000000000001.bin"),
    )
    .unwrap();
}

fn req() -> WorldMigrationRequest {
    WorldMigrationRequest {
        source_node: FederationNodeId::new([1; 32]),
        destination_node: FederationNodeId::new([2; 32]),
        world_id: "world-1".into(),
        expected_package_root: [7; 32],
        expected_checkpoint_root: [0; 32],
    }
}

#[test]
fn test_world_freeze_success() {
    let t = tempfile::tempdir().unwrap();
    seed_world(t.path());
    let d = freeze_world(t.path(), "w").unwrap();
    assert!(d.frozen);
}
#[test]
fn test_world_resume_success() {
    let t = tempfile::tempdir().unwrap();
    seed_world(t.path());
    freeze_world(t.path(), "w").unwrap();
    let d = resume_world(t.path(), "w").unwrap();
    assert!(!d.frozen);
}
#[test]
fn test_world_migration_success() {
    let s = tempfile::tempdir().unwrap();
    let b = tempfile::tempdir().unwrap();
    let d = tempfile::tempdir().unwrap();
    seed_world(s.path());
    let mut r = req();
    let m = execution_core::canonical::load_manifest(&fixture("manifest.bin")).unwrap();
    r.expected_package_root = m.package_root;
    r.expected_checkpoint_root = m.checkpoint_root;
    let out = migrate_world(s.path(), b.path(), d.path(), &r).unwrap();
    assert!(out.migration_ok);
}
#[test]
fn test_world_migration_package_mismatch_fails() {
    let s = tempfile::tempdir().unwrap();
    let b = tempfile::tempdir().unwrap();
    let d = tempfile::tempdir().unwrap();
    seed_world(s.path());
    let r = req();
    let out = migrate_world(s.path(), b.path(), d.path(), &r).unwrap();
    assert!(!out.migration_ok);
}
#[test]
fn test_world_migration_checkpoint_mismatch_fails() {
    let s = tempfile::tempdir().unwrap();
    let b = tempfile::tempdir().unwrap();
    let d = tempfile::tempdir().unwrap();
    seed_world(s.path());
    let mut r = req();
    let m = execution_core::canonical::load_manifest(&fixture("manifest.bin")).unwrap();
    r.expected_package_root = m.package_root;
    let out = migrate_world(s.path(), b.path(), d.path(), &r).unwrap();
    assert!(!out.migration_ok);
}
#[test]
fn test_world_migration_lineage_mismatch_fails() {
    let s = tempfile::tempdir().unwrap();
    let b = tempfile::tempdir().unwrap();
    let d = tempfile::tempdir().unwrap();
    seed_world(s.path());
    fs::write(s.path().join("lineage.bin"), b"bad").unwrap();
    let mut r = req();
    let m = execution_core::canonical::load_manifest(&fixture("manifest.bin")).unwrap();
    r.expected_package_root = m.package_root;
    r.expected_checkpoint_root = m.checkpoint_root;
    assert!(migrate_world(s.path(), b.path(), d.path(), &r).is_err());
}
#[test]
fn test_world_migration_replay_mismatch_fails() {
    let s = tempfile::tempdir().unwrap();
    let b = tempfile::tempdir().unwrap();
    let d = tempfile::tempdir().unwrap();
    seed_world(s.path());
    let mut x = fs::read(s.path().join("receipts/receipt_0000000000000001.bin")).unwrap();
    x[0] ^= 1;
    fs::write(s.path().join("receipts/receipt_0000000000000001.bin"), x).unwrap();
    let mut r = req();
    let m = execution_core::canonical::load_manifest(&fixture("manifest.bin")).unwrap();
    r.expected_package_root = m.package_root;
    r.expected_checkpoint_root = m.checkpoint_root;
    assert!(migrate_world(s.path(), b.path(), d.path(), &r).is_err());
}
#[test]
fn test_world_resume_preserves_sequence() {
    let t = tempfile::tempdir().unwrap();
    seed_world(t.path());
    let f = freeze_world(t.path(), "w").unwrap();
    let r = resume_world(t.path(), "w").unwrap();
    assert_eq!(f.latest_sequence, r.latest_sequence);
}
#[test]
fn test_world_resume_preserves_replay_root() {
    let t = tempfile::tempdir().unwrap();
    seed_world(t.path());
    let m = execution_core::canonical::load_manifest(&t.path().join("manifest.bin")).unwrap();
    resume_world(t.path(), "w").unwrap();
    let m2 = execution_core::canonical::load_manifest(&t.path().join("manifest.bin")).unwrap();
    assert_eq!(m.final_state_root, m2.final_state_root);
}
#[test]
fn test_world_migration_is_deterministic() {
    let s = tempfile::tempdir().unwrap();
    seed_world(s.path());
    let mut r = req();
    let m = execution_core::canonical::load_manifest(&fixture("manifest.bin")).unwrap();
    r.expected_package_root = m.package_root;
    r.expected_checkpoint_root = m.checkpoint_root;
    let b1 = tempfile::tempdir().unwrap();
    let d1 = tempfile::tempdir().unwrap();
    let b2 = tempfile::tempdir().unwrap();
    let d2 = tempfile::tempdir().unwrap();
    let a = migrate_world(s.path(), b1.path(), d1.path(), &r).unwrap();
    seed_world(s.path());
    let c = migrate_world(s.path(), b2.path(), d2.path(), &r).unwrap();
    assert_eq!(a, c);
}
