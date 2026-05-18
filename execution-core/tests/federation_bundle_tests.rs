mod common;

use std::{fs, path::PathBuf};

use execution_core::federation::bundle::{
    export_continuity_bundle, import_continuity_bundle, verify_continuity_bundle,
};

fn fixture(name: &str) -> PathBuf {
    common::fixtures::ensure_repo_counter_world_fixtures();
    common::fixtures::repo_counter_world_fixture_dir().join(name)
}

fn receipts() -> Vec<PathBuf> {
    vec![fixture("receipt_1.bin"), fixture("receipt_2.bin")]
}

#[test]
fn test_export_bundle_success() {
    let t = tempfile::tempdir().unwrap();
    let out = export_continuity_bundle(
        t.path(),
        &fixture("world.wasm"),
        &fixture("checkpoint_0.bin"),
        &fixture("lineage.bin"),
        &receipts(),
        &fixture("manifest.bin"),
        &fixture("recovery_descriptor.bin"),
    )
    .unwrap();
    assert!(out.bundle_ok);
}

#[test]
fn test_verify_bundle_success() {
    let t = tempfile::tempdir().unwrap();
    export_continuity_bundle(
        t.path(),
        &fixture("world.wasm"),
        &fixture("checkpoint_0.bin"),
        &fixture("lineage.bin"),
        &receipts(),
        &fixture("manifest.bin"),
        &fixture("recovery_descriptor.bin"),
    )
    .unwrap();
    assert!(verify_continuity_bundle(t.path()).unwrap().bundle_ok);
}
#[test]
fn test_import_bundle_success() {
    let src = tempfile::tempdir().unwrap();
    let dst = tempfile::tempdir().unwrap();
    export_continuity_bundle(
        src.path(),
        &fixture("world.wasm"),
        &fixture("checkpoint_0.bin"),
        &fixture("lineage.bin"),
        &receipts(),
        &fixture("manifest.bin"),
        &fixture("recovery_descriptor.bin"),
    )
    .unwrap();
    assert!(
        import_continuity_bundle(src.path(), dst.path())
            .unwrap()
            .bundle_ok
    );
}

fn tamper_and_fail(path: &std::path::Path) {
    let mut b = fs::read(path).unwrap();
    b[0] ^= 1;
    fs::write(path, b).unwrap();
    assert!(
        verify_continuity_bundle(path.parent().unwrap_or(path)).is_err()
            || !verify_continuity_bundle(path.parent().unwrap_or(path))
                .unwrap()
                .bundle_ok
    );
}

#[test]
fn test_bundle_package_tamper_fails() {
    let t = tempfile::tempdir().unwrap();
    export_continuity_bundle(
        t.path(),
        &fixture("world.wasm"),
        &fixture("checkpoint_0.bin"),
        &fixture("lineage.bin"),
        &receipts(),
        &fixture("manifest.bin"),
        &fixture("recovery_descriptor.bin"),
    )
    .unwrap();
    tamper_and_fail(&t.path().join("package/world.wasm"));
}
#[test]
fn test_bundle_checkpoint_tamper_fails() {
    let t = tempfile::tempdir().unwrap();
    export_continuity_bundle(
        t.path(),
        &fixture("world.wasm"),
        &fixture("checkpoint_0.bin"),
        &fixture("lineage.bin"),
        &receipts(),
        &fixture("manifest.bin"),
        &fixture("recovery_descriptor.bin"),
    )
    .unwrap();
    tamper_and_fail(&t.path().join("checkpoint.bin"));
}
#[test]
fn test_bundle_lineage_tamper_fails() {
    let t = tempfile::tempdir().unwrap();
    export_continuity_bundle(
        t.path(),
        &fixture("world.wasm"),
        &fixture("checkpoint_0.bin"),
        &fixture("lineage.bin"),
        &receipts(),
        &fixture("manifest.bin"),
        &fixture("recovery_descriptor.bin"),
    )
    .unwrap();
    tamper_and_fail(&t.path().join("lineage.bin"));
}
#[test]
fn test_bundle_receipt_tamper_fails() {
    let t = tempfile::tempdir().unwrap();
    export_continuity_bundle(
        t.path(),
        &fixture("world.wasm"),
        &fixture("checkpoint_0.bin"),
        &fixture("lineage.bin"),
        &receipts(),
        &fixture("manifest.bin"),
        &fixture("recovery_descriptor.bin"),
    )
    .unwrap();
    tamper_and_fail(&t.path().join("receipts/receipt_0000000000000001.bin"));
}
#[test]
fn test_bundle_manifest_tamper_fails() {
    let t = tempfile::tempdir().unwrap();
    export_continuity_bundle(
        t.path(),
        &fixture("world.wasm"),
        &fixture("checkpoint_0.bin"),
        &fixture("lineage.bin"),
        &receipts(),
        &fixture("manifest.bin"),
        &fixture("recovery_descriptor.bin"),
    )
    .unwrap();
    tamper_and_fail(&t.path().join("manifest.bin"));
}
#[test]
fn test_bundle_descriptor_tamper_fails() {
    let t = tempfile::tempdir().unwrap();
    export_continuity_bundle(
        t.path(),
        &fixture("world.wasm"),
        &fixture("checkpoint_0.bin"),
        &fixture("lineage.bin"),
        &receipts(),
        &fixture("manifest.bin"),
        &fixture("recovery_descriptor.bin"),
    )
    .unwrap();
    tamper_and_fail(&t.path().join("descriptor.bin"));
}

#[test]
fn test_bundle_receipt_order_is_deterministic() {
    let t = tempfile::tempdir().unwrap();
    let reverse = vec![fixture("receipt_2.bin"), fixture("receipt_1.bin")];
    export_continuity_bundle(
        t.path(),
        &fixture("world.wasm"),
        &fixture("checkpoint_0.bin"),
        &fixture("lineage.bin"),
        &reverse,
        &fixture("manifest.bin"),
        &fixture("recovery_descriptor.bin"),
    )
    .unwrap();
    assert!(t
        .path()
        .join("receipts/receipt_0000000000000000.bin")
        .exists());
    assert!(t
        .path()
        .join("receipts/receipt_0000000000000001.bin")
        .exists());
}

#[test]
fn test_bundle_layout_is_deterministic() {
    let a = tempfile::tempdir().unwrap();
    let b = tempfile::tempdir().unwrap();
    export_continuity_bundle(
        a.path(),
        &fixture("world.wasm"),
        &fixture("checkpoint_0.bin"),
        &fixture("lineage.bin"),
        &receipts(),
        &fixture("manifest.bin"),
        &fixture("recovery_descriptor.bin"),
    )
    .unwrap();
    export_continuity_bundle(
        b.path(),
        &fixture("world.wasm"),
        &fixture("checkpoint_0.bin"),
        &fixture("lineage.bin"),
        &receipts(),
        &fixture("manifest.bin"),
        &fixture("recovery_descriptor.bin"),
    )
    .unwrap();
    for p in [
        "manifest.bin",
        "lineage.bin",
        "checkpoint.bin",
        "descriptor.bin",
        "package/world.wasm",
        "receipts/receipt_0000000000000000.bin",
        "receipts/receipt_0000000000000001.bin",
    ] {
        assert_eq!(
            fs::read(a.path().join(p)).unwrap(),
            fs::read(b.path().join(p)).unwrap()
        );
    }
}
