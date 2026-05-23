use execution_core::coordination::capability::{negotiate_capabilities, NodeCapabilities};
use execution_core::coordination::compatibility::ProtocolCompatibility;
use execution_core::coordination::epoch::FederationEpochManifest;
use execution_core::coordination::governance::GovernanceManifest;
use execution_core::coordination::migration::{MigrationJournal, MigrationTransition};
use execution_core::coordination::persistence::SovereignPersistenceManifest;
use execution_core::coordination::replay_continuity::validate_upgrade_safe_replay;
use execution_core::coordination::routing::route_execution;
use execution_core::coordination::topology::{FederationTopologyManifest, TopologyNode};
use execution_core::coordination::upgrade::ProtocolUpgradeManifest;

#[test]
fn deterministic_coordination_hashes_and_validation() {
    let upgrade = ProtocolUpgradeManifest {
        protocol_version: "2".into(),
        previous_protocol_version: "1".into(),
        upgrade_epoch: 2,
        compatibility_hash: "c".into(),
        migration_hash: "m".into(),
        replay_root_before: "rb".into(),
        replay_root_after: "ra".into(),
        continuity_hash: "k".into(),
    };
    assert_eq!(upgrade.canonical_hash(), upgrade.canonical_hash());

    let epoch = FederationEpochManifest {
        epoch_id: 2,
        federation_manifest_hash: "f".into(),
        checkpoint_hash: "cp".into(),
        topology_hash: "t".into(),
        capability_hash: "cap".into(),
        replay_root: "r".into(),
        continuity_root: "cont".into(),
    };
    assert_eq!(epoch.canonical_hash(), epoch.canonical_hash());

    let governance = GovernanceManifest {
        protocol_version: "2".into(),
        federation_epoch: 2,
        capability_root: "cap".into(),
        topology_root: "top".into(),
        migration_root: "mig".into(),
        replay_root: "rep".into(),
    };
    assert_eq!(governance.canonical_hash(), governance.canonical_hash());

    let mut comp = ProtocolCompatibility {
        supported_protocol_versions: vec!["2".into(), "1".into(), "2".into()],
        abi_compatibility_hash: "a".into(),
        execution_compatibility_hash: "e".into(),
        migration_compatibility_hash: "m".into(),
        capability_negotiation_root: "n".into(),
    };
    comp.canonicalize();
    assert!(comp.is_protocol_supported("2").is_ok());

    let nodes = vec![
        NodeCapabilities {
            node_id: "n1".into(),
            protocol_versions: vec!["2".into()],
            abi_versions: vec!["abi1".into()],
            wasm_runtime_versions: vec!["w1".into()],
            federation_features: vec!["dag".into(), "replay".into()],
            replay_compatibility: true,
        },
        NodeCapabilities {
            node_id: "n2".into(),
            protocol_versions: vec!["2".into(), "3".into()],
            abi_versions: vec!["abi1".into()],
            wasm_runtime_versions: vec!["w1".into()],
            federation_features: vec!["replay".into()],
            replay_compatibility: true,
        },
    ];
    let negotiated = negotiate_capabilities(&nodes);
    assert_eq!(negotiated.shared_protocol_versions, vec!["2".to_string()]);

    let mut topology = FederationTopologyManifest {
        federation_epoch: 2,
        nodes: vec![
            TopologyNode {
                node_id: "b".into(),
                node_continuity_hash: "2".into(),
            },
            TopologyNode {
                node_id: "a".into(),
                node_continuity_hash: "1".into(),
            },
        ],
        federation_routing_hash: "rh".into(),
        capability_root: "cap".into(),
    };
    topology.canonicalize();
    assert_eq!(topology.nodes[0].node_id, "a");

    let routes = route_execution(
        &[("w1".into(), "2".into())],
        &["a".into(), "b".into()],
        &negotiated,
    );
    assert_eq!(routes.routes.len(), 1);

    let mut journal = MigrationJournal {
        upgrades: vec![],
        transitions: vec![],
    };
    journal.append_upgrade(
        upgrade.clone(),
        MigrationTransition {
            replay_transition: "r".into(),
            capability_transition: "c".into(),
            topology_transition: "t".into(),
            checkpoint_transition: "cp".into(),
        },
    );
    assert!(validate_upgrade_safe_replay(&[epoch.clone(), epoch], &journal).is_ok());

    let persisted = SovereignPersistenceManifest {
        sovereign_identity: "id".into(),
        continuity_root: "cont".into(),
        migration_lineage_root: "m".into(),
        checkpoint_lineage_root: "c".into(),
        replay_restoration_root: "r".into(),
    };
    assert!(persisted.verify_restoration(&persisted));
}
