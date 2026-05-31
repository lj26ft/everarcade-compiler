use execution_core::deployment::evernode_runtime::{
    certify_federation, certify_recovery, package_artifact, validate_load_gate,
    validate_multi_node_federation, verify_deployment, verify_package_artifact,
    EverNodeDeploymentRoots, EverNodeFederationNode, LoadValidationPlan, MultiNodeFederationPlan,
};
use execution_core::xrpl::publisher::{
    AnchorPublisher, CheckpointPublisher, DeploymentAnchorRecord, DeploymentPublisher,
    ReceiptAnchorRecord, ReceiptPublisher, ReplayAnchorRecord, ReplayPublisher, WorldAnchorRecord,
};
use std::collections::BTreeSet;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Once;
use std::thread;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("execution-core has workspace parent")
        .to_path_buf()
}

fn read(relative: &str) -> String {
    fs::read_to_string(root().join(relative)).unwrap_or_else(|err| panic!("read {relative}: {err}"))
}

fn bytes(relative: &str) -> Vec<u8> {
    fs::read(root().join(relative)).unwrap_or_else(|err| panic!("read {relative}: {err}"))
}

fn assert_exists(relative: &str) {
    assert!(root().join(relative).exists(), "missing {relative}");
}

fn assert_contains(relative: &str, needle: &str) {
    let body = read(relative);
    assert!(body.contains(needle), "{relative} missing {needle}");
}

static PACKAGE_GENERATION: Once = Once::new();

fn ensure_generated_packages() {
    PACKAGE_GENERATION.call_once(|| {
        let status = Command::new("bash")
            .arg("scripts/generate_evernode_packages.sh")
            .current_dir(root())
            .status()
            .expect("run package generation script");
        assert!(status.success(), "package generation failed");
    });
}

fn roots() -> EverNodeDeploymentRoots {
    EverNodeDeploymentRoots {
        replay_root: "replay:arena-vanguard:tick:3:root".to_owned(),
        world_root: "world:arena-vanguard:tick:3:players:2:marketplace:6".to_owned(),
        checkpoint_root: "checkpoint:arena-vanguard:tick:3:root".to_owned(),
        continuity_root: "continuity:arena-vanguard:deployment:root".to_owned(),
    }
}

#[test]
fn test_runtime_package_generation() {
    ensure_generated_packages();
    for artifact in [
        "deployment/evernode/runtime/arena-vanguard-runtime.tar.gz",
        "deployment/evernode/runtime/arena-vanguard-runtime.tar.gz.sha256",
        "deployment/evernode/runtime/arena-vanguard-runtime.tar.gz.sig",
        "deployment/evernode/runtime/arena-vanguard-runtime.receipt.json",
    ] {
        assert_exists(artifact);
    }
    let artifact = package_artifact(
        "arena-vanguard-runtime.tar.gz",
        bytes("deployment/evernode/runtime/arena-vanguard-runtime.tar.gz"),
    );
    assert!(verify_package_artifact(&artifact));
}

#[test]
fn test_deployment_manifest_generation() {
    for manifest in [
        "deployment/evernode/runtime/runtime-manifest.toml",
        "deployment/evernode/runtime/world-manifest.toml",
        "deployment/evernode/runtime/deployment-manifest.toml",
        "deployment/evernode/runtime/operator-manifest.toml",
    ] {
        assert_exists(manifest);
        assert_contains(manifest, "reproducible = true");
    }
    let verification = verify_deployment(&roots());
    assert!(verification.package_verified);
    assert!(verification.deployment_verified);
    assert!(verification.replay_continuity_verified);
    assert!(verification.checkpoint_continuity_verified);
}

#[test]
fn test_operator_bootstrap() {
    for guide in [
        "install",
        "bootstrap",
        "deploy",
        "recover",
        "upgrade",
        "rollback",
    ] {
        assert_exists(&format!("docs/operators/{guide}.md"));
    }
    assert_contains("docs/operators/bootstrap.md", "replay root");
    assert_contains(
        "deployment/evernode/runtime/operator-manifest.toml",
        "default_cargo_build_jobs = 1",
    );
}

#[test]
fn test_checkpoint_restore_after_deploy() {
    let before = roots();
    let after = roots();
    for scenario in [
        "runtime crash",
        "node restart",
        "checkpoint restore",
        "world restore",
    ] {
        let report = certify_recovery(scenario, &before, &after);
        assert!(report.same_replay_root, "{scenario} replay root changed");
        assert!(report.same_world_root, "{scenario} world root changed");
        assert!(
            report.same_continuity_root,
            "{scenario} continuity root changed"
        );
    }
}

#[test]
fn test_federated_deployment_join() {
    let report = certify_federation("evernode-a", "evernode-b");
    assert!(report.joined);
    assert!(report.checkpoint_synced);
    assert!(report.replay_synced);
    assert!(report.recovery_verified);
    assert_contains(
        "deployment/evernode/runtime/deployment-manifest.toml",
        "transport = \"tcp\"",
    );
}

#[test]
fn test_multinode_federation_load_gate() {
    const NODE_COUNT: usize = 4;
    const MESSAGES_PER_NODE: u64 = 32;

    assert_contains(
        "deployment/evernode/runtime/multinode-federation-load-manifest.toml",
        "required_nodes = 4",
    );
    assert_contains(
        "deployment/reports/multinode_federation_load_gate.md",
        "load balance | Ready",
    );

    let mut listeners = Vec::new();
    for _ in 0..NODE_COUNT {
        listeners.push(TcpListener::bind(("127.0.0.1", 0)).expect("bind federation node"));
    }

    let addresses: Vec<_> = listeners
        .iter()
        .map(|listener| listener.local_addr().expect("local addr"))
        .collect();

    let handles: Vec<_> = listeners
        .into_iter()
        .enumerate()
        .map(|(idx, listener)| {
            let endpoint = addresses[idx].to_string();
            thread::spawn(move || {
                let (mut stream, _) = listener.accept().expect("accept federation probe");
                let mut payload = String::new();
                stream
                    .read_to_string(&mut payload)
                    .expect("read federation probe");
                assert!(payload.contains("arena-vanguard-runtime-package-root"));
                assert!(payload.contains("replay:arena-vanguard:tick:3:root"));
                EverNodeFederationNode {
                    node_id: format!("evernode-{}", idx + 1),
                    endpoint,
                    package_hash: "arena-vanguard-runtime-package-root".to_owned(),
                    replay_root: "replay:arena-vanguard:tick:3:root".to_owned(),
                    checkpoint_root: "checkpoint:arena-vanguard:tick:3:root".to_owned(),
                    continuity_root: "continuity:arena-vanguard:deployment:root".to_owned(),
                    observed_messages: MESSAGES_PER_NODE,
                    max_supported_messages: 64,
                }
            })
        })
        .collect();

    for (idx, address) in addresses.iter().enumerate() {
        let mut stream = TcpStream::connect(address).expect("connect federation node");
        write!(
            stream,
            "node={idx};package=arena-vanguard-runtime-package-root;replay=replay:arena-vanguard:tick:3:root;checkpoint=checkpoint:arena-vanguard:tick:3:root;continuity=continuity:arena-vanguard:deployment:root"
        )
        .expect("write federation probe");
    }

    let nodes: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().expect("node thread joins"))
        .collect();

    let federation = validate_multi_node_federation(&MultiNodeFederationPlan {
        world_id: "arena-vanguard".to_owned(),
        required_nodes: NODE_COUNT,
        quorum: 3,
        nodes: nodes.clone(),
    });
    assert_eq!(federation.node_count, NODE_COUNT);
    assert!(federation.joined);
    assert!(federation.checkpoint_synced);
    assert!(federation.replay_synced);
    assert!(federation.recovery_verified);

    let load = validate_load_gate(
        &LoadValidationPlan {
            expected_nodes: NODE_COUNT,
            expected_messages_per_node: MESSAGES_PER_NODE,
            max_messages_per_node: 64,
        },
        &nodes,
    );
    assert_eq!(load.total_messages, NODE_COUNT as u64 * MESSAGES_PER_NODE);
    assert!(load.balanced);
    assert!(load.within_capacity);
    assert_eq!(load.deterministic_root.len(), 64);
}

#[test]
fn test_anchor_record_generation() {
    let records = BTreeSet::from([
        ReceiptPublisher.record_type(),
        ReplayPublisher.record_type(),
        CheckpointPublisher.record_type(),
        DeploymentPublisher.record_type(),
    ]);
    assert_eq!(
        records,
        BTreeSet::from([
            "DeploymentAnchorRecord",
            "ReceiptAnchorRecord",
            "ReplayAnchorRecord",
            "WorldAnchorRecord",
        ])
    );
    assert_exists("xrpl/publisher/README.md");
}

#[test]
fn test_anchor_payload_verification() {
    let roots = roots();
    let receipt = ReceiptAnchorRecord {
        receipt_root: "receipt:arena-vanguard:root".to_owned(),
        replay_root: roots.replay_root.clone(),
        continuity_root: roots.continuity_root.clone(),
    };
    let replay = ReplayAnchorRecord {
        replay_root: roots.replay_root.clone(),
        checkpoint_root: roots.checkpoint_root.clone(),
        continuity_root: roots.continuity_root.clone(),
    };
    let world = WorldAnchorRecord {
        world_root: roots.world_root.clone(),
        replay_root: roots.replay_root.clone(),
        continuity_root: roots.continuity_root.clone(),
    };
    let deployment = DeploymentAnchorRecord {
        deployment_root: "deployment:arena-vanguard:root".to_owned(),
        package_hash: "package:arena-vanguard:root".to_owned(),
        continuity_root: roots.continuity_root.clone(),
    };
    assert!(ReceiptPublisher.verify_payload(&ReceiptPublisher.publication_payload(&receipt)));
    assert!(ReplayPublisher.verify_payload(&ReplayPublisher.publication_payload(&replay)));
    assert!(CheckpointPublisher.verify_payload(&CheckpointPublisher.publication_payload(&world)));
    assert!(
        DeploymentPublisher.verify_payload(&DeploymentPublisher.publication_payload(&deployment))
    );

    let replayed = ReceiptPublisher.publication_payload(&receipt);
    assert_eq!(
        replayed.record_hash,
        ReceiptPublisher.publication_payload(&receipt).record_hash
    );
}

#[test]
fn test_arena_vanguard_deployment_package() {
    ensure_generated_packages();
    for name in ["runtime", "world", "deployment"] {
        assert_exists(&format!(
            "deployment/evernode/runtime/arena-vanguard-{name}.tar.gz"
        ));
        assert_exists(&format!(
            "deployment/evernode/runtime/arena-vanguard-{name}.tar.gz.sha256"
        ));
        assert_exists(&format!(
            "deployment/evernode/runtime/arena-vanguard-{name}.tar.gz.sig"
        ));
        assert_exists(&format!(
            "deployment/evernode/runtime/arena-vanguard-{name}.receipt.json"
        ));
        assert_contains(
            &format!("deployment/evernode/runtime/arena-vanguard-{name}.receipt.json"),
            "\"reproducible\":true",
        );
    }
}

#[test]
fn test_arena_vanguard_deployment_recovery() {
    assert_contains(
        "deployment/reports/operator_readiness_report.md",
        "Recovery Workflow",
    );
    assert_contains(
        "deployment/reports/arena_vanguard_launch_candidate.md",
        "Recovery Workflow",
    );
    let report = certify_recovery("deployment recovery", &roots(), &roots());
    assert!(report.same_replay_root && report.same_world_root && report.same_continuity_root);
}

#[test]
fn test_launch_candidate_gate() {
    assert_contains(
        "deployment/reports/evernode_launch_gate.md",
        "| runtime | Ready |",
    );
    assert_contains(
        "deployment/reports/evernode_launch_gate.md",
        "| operator tooling | Partially Ready |",
    );
    assert_contains(
        "deployment/reports/xrpl_launch_gate.md",
        "| publication | Partially Ready |",
    );
    assert_contains(
        "deployment/reports/arena_vanguard_launch_candidate.md",
        "Operator Workflow",
    );
    assert_contains(
        "deployment/reports/multinode_federation_load_gate.md",
        "Multi-Node Federation and Load Gate",
    );
}
