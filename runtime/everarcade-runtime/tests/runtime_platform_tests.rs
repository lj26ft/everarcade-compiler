use everarcade_runtime::*;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn package(root: &Path, version: &str, valid: bool) -> PathBuf {
    let dir = root.join(format!("pkg-{version}"));
    fs::create_dir_all(&dir).unwrap();
    let wasm = format!("wasm-{version}").into_bytes();
    fs::write(dir.join("world.wasm"), &wasm).unwrap();
    let hash = hex::encode(Sha256::digest(&wasm));
    let manifest = PackageManifest {
        package_id: "test-package".into(),
        package_version: version.into(),
        runtime_compatibility: if valid {
            RUNTIME_VERSION.into()
        } else {
            "old-runtime".into()
        },
        wasm_path: "world.wasm".into(),
        wasm_hash: hash.clone(),
        signature: format!("sha256:{hash}"),
        world_id: "world-1".into(),
    };
    fs::write(
        dir.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .unwrap();
    fs::write(
        dir.join("world.json"),
        br#"{"world_id":"world-1","name":"Test World"}"#,
    )
    .unwrap();
    dir
}

fn config(tmp: &TempDir) -> RuntimeConfiguration {
    let pkg = package(&tmp.path().join("packages-src"), "1.0.0", true);
    let mut c = RuntimeConfiguration::new(tmp.path().join("data"), "world-1", pkg);
    c.checkpoint_interval_ticks = 2;
    c
}

fn boot_with_ticks(tmp: &TempDir, ticks: u64) -> RuntimeLoop {
    let mut rt = RuntimeLoop::boot(config(tmp)).unwrap();
    for i in 0..ticks {
        rt.submit_input("test", format!("input-{i}").into_bytes())
            .unwrap();
    }
    assert_eq!(rt.run_ticks(ticks).unwrap(), ticks);
    rt
}

#[test]
fn test_runtime_boot() {
    let tmp = TempDir::new().unwrap();
    let rt = RuntimeLoop::boot(config(&tmp)).unwrap();
    assert_eq!(rt.lifecycle.state, RuntimeState::Running);
}

#[test]
fn test_package_validation() {
    let tmp = TempDir::new().unwrap();
    assert!(PackageLoader::new(RUNTIME_VERSION)
        .load(package(tmp.path(), "bad", false))
        .is_err());
    assert!(PackageLoader::new(RUNTIME_VERSION)
        .load(package(tmp.path(), "ok", true))
        .is_ok());
}

#[test]
fn test_tick_execution() {
    let tmp = TempDir::new().unwrap();
    let rt = boot_with_ticks(&tmp, 1);
    assert_eq!(rt.metrics.ticks_executed, 1);
    assert!(rt.health.latest_receipt.is_some());
}

#[test]
fn test_journal_integrity() {
    let tmp = TempDir::new().unwrap();
    let rt = boot_with_ticks(&tmp, 3);
    assert_eq!(rt.journal.verify().unwrap().unwrap().sequence, 3);
}

#[test]
fn test_checkpoint_restore() {
    let tmp = TempDir::new().unwrap();
    let rt = boot_with_ticks(&tmp, 2);
    let cp = rt.checkpoints.latest().unwrap().unwrap();
    rt.checkpoints.verify_checkpoint(&cp).unwrap();
    assert_eq!(cp.manifest.journal_position, 2);
}

#[test]
fn test_replay_equivalence() {
    let tmp = TempDir::new().unwrap();
    let rt = boot_with_ticks(&tmp, 2);
    let entries = rt.journal.entries().unwrap();
    let root = ReplayManager::replay_root(&[], &entries);
    assert!(ReplayManager
        .verify_equivalence(&[], &entries, &root)
        .is_ok());
}

#[test]
fn test_crash_recovery() {
    let tmp = TempDir::new().unwrap();
    let rt = boot_with_ticks(&tmp, 2);
    let report = RecoveryManager {
        checkpoint_manager: rt.checkpoints,
        journal_manager: rt.journal,
        replay_manager: ReplayManager,
    }
    .recover()
    .unwrap();
    assert_eq!(report.status, "recovered");
}

#[test]
fn test_backup_restore() {
    let tmp = TempDir::new().unwrap();
    let rt = boot_with_ticks(&tmp, 2);
    let manager = BackupManager {
        dir: rt.config.backups_dir(),
        checkpoints: rt.checkpoints,
        persistence: rt.persistence,
    };
    let manifest = manager.backup().unwrap();
    assert_eq!(
        manager.verify(&manifest.backup_id).unwrap().checkpoint_hash,
        manifest.checkpoint_hash
    );
}

#[test]
fn test_upgrade_rollback() {
    let tmp = TempDir::new().unwrap();
    let rt = boot_with_ticks(&tmp, 2);
    let bad_pkg = package(&tmp.path().join("packages-src"), "2.0.0", false);
    let manager = UpgradeManager {
        backup: BackupManager {
            dir: rt.config.backups_dir(),
            checkpoints: rt.checkpoints,
            persistence: rt.persistence,
        },
        loader: PackageLoader::new(RUNTIME_VERSION),
    };
    let report = manager.upgrade(bad_pkg, None).unwrap();
    assert!(report.rolled_back);
}

#[test]
fn test_state_root_stability() {
    let tmp = TempDir::new().unwrap();
    let rt = boot_with_ticks(&tmp, 2);
    let first = rt.health.world_root.clone();
    let entries = rt.journal.entries().unwrap();
    assert_eq!(entries.last().unwrap().state_root, first);
}

#[test]
fn test_persistence_corruption_detection() {
    let tmp = TempDir::new().unwrap();
    let p = PersistenceManager::new(tmp.path());
    let file = tmp.path().join("value.json");
    p.write_versioned(&file, &serde_json::json!({"a":1}))
        .unwrap();
    let mut v: serde_json::Value = serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
    v["payload"]["a"] = 2.into();
    fs::write(&file, serde_json::to_vec(&v).unwrap()).unwrap();
    assert!(p.read_versioned::<serde_json::Value>(&file).is_err());
}

#[test]
fn test_runtime_shutdown_recovery() {
    let tmp = TempDir::new().unwrap();
    let mut rt = boot_with_ticks(&tmp, 2);
    rt.stop().unwrap();
    let recovered = RuntimeLoop::boot(config(&tmp)).unwrap();
    assert_eq!(recovered.lifecycle.state, RuntimeState::Running);
}
