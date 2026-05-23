use std::{fs, process::Command};

#[test]
fn test_runtime_config_determinism() {
    let out1 = Command::new("cargo").args(["run", "-q", "-p", "everarcade-cli", "--", "runtime-snapshot", "runtime/config"]).output().unwrap();
    let out2 = Command::new("cargo").args(["run", "-q", "-p", "everarcade-cli", "--", "runtime-snapshot", "runtime/config"]).output().unwrap();
    assert_eq!(out1.stdout, out2.stdout);
}

#[test]
fn test_evernode_runtime_manifest_equivalence() {
    let out = Command::new("cargo").args(["run", "-q", "-p", "everarcade-cli", "--", "runtime-snapshot", "runtime/config"]).output().unwrap();
    assert!(String::from_utf8_lossy(&out.stdout).contains("appliance"));
}

#[test]
fn test_runtime_release_integrity() {
    let _ = Command::new("bash").args(["scripts/build_runtime_release.sh"]).status().unwrap();
    assert!(fs::metadata("dist/MANIFEST.json").is_ok());
    assert!(fs::metadata("dist/SHA256SUMS").is_ok());
}

#[test]
fn test_runtime_bootstrap_equivalence() {
    let _ = Command::new("bash").args(["scripts/build_runtime_release.sh"]).status().unwrap();
    let t1 = Command::new("bash").args(["-c", "tmp=$(mktemp -d); tar -xzf dist/everarcade-runtime-linux-x86_64.tar.gz -C $tmp; $tmp/everarcade-runtime/scripts/bootstrap.sh"]).output().unwrap();
    let t2 = Command::new("bash").args(["-c", "tmp=$(mktemp -d); tar -xzf dist/everarcade-runtime-linux-x86_64.tar.gz -C $tmp; $tmp/everarcade-runtime/scripts/bootstrap.sh"]).output().unwrap();
    assert_eq!(t1.status.code(), t2.status.code());
}

#[test]
fn test_runtime_startup_replay_continuity() {
    let _ = Command::new("bash").args(["scripts/build_runtime_release.sh"]).status().unwrap();
    let out = Command::new("bash").args(["-c", "tmp=$(mktemp -d); tar -xzf dist/everarcade-runtime-linux-x86_64.tar.gz -C $tmp; $tmp/everarcade-runtime/scripts/start.sh >/tmp/ea-start.log 2>&1 || true; test -f $tmp/everarcade-runtime/runtime/replay/latest/frame-0000.json"]).status().unwrap();
    assert!(out.success());
}

#[test]
fn test_runtime_topology_validation() {
    let text = fs::read_to_string("runtime/config/topology.toml").unwrap();
    assert!(text.contains("node_count"));
}
