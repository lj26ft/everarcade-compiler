use execution_core::operations::{
    appliance::ApplianceManifest,
    archive_ops::ArchiveManifest,
    diagnostics::DiagnosticManifest,
    evernode::EvernodeDeploymentManifest,
    lifecycle::{LifecycleJournal, LifecycleTransitionKind},
    operator::inspect_continuity,
    recovery_ops::RecoveryManifest,
    scheduler::{ScheduledTick, SchedulingManifest},
    sdk::SdkProjectManifest,
    sharding::{TopologyManifest, WorldPartition},
};

#[test]
fn sovereign_operations_determinism_suite() {
    let appliance = ApplianceManifest::new(
        "pkg".into(),
        "topo".into(),
        "replay".into(),
        "checkpoint".into(),
        "persist".into(),
    );
    assert!(appliance.verify_orchestration_root());

    let mut journal = LifecycleJournal::default();
    journal.append(LifecycleTransitionKind::Bootstrap, "boot".into());
    journal.append(LifecycleTransitionKind::Activation, "active".into());
    assert_eq!(journal.root(), journal.root());

    let topology = TopologyManifest {
        world_id: "world".into(),
        partitions: vec![WorldPartition {
            partition_id: "p0".into(),
            routing_boundary: "north".into(),
            sync_window: 5,
        }],
        prior_topology_root: "genesis".into(),
    };
    assert_eq!(topology.shard_lineage_hash(), topology.shard_lineage_hash());

    let diag = inspect_continuity(&appliance, &topology);
    assert!(diag.replay_verified);

    let archive = ArchiveManifest {
        era: 1,
        previous_archive_root: "a0".into(),
        compression_profile: "zstd-1".into(),
        restoration_checkpoint: "cp".into(),
        retention_policy: "long".into(),
    };
    assert_eq!(archive.continuity_root(), archive.continuity_root());

    let recovery = RecoveryManifest {
        checkpoint_restoration: "cp".into(),
        replay_restoration: "rp".into(),
        topology_restoration: "tp".into(),
        settlement_restoration: "st".into(),
        archive_restoration: "ar".into(),
    };
    assert_eq!(recovery.recovery_root(), recovery.recovery_root());

    let schedule = SchedulingManifest {
        federation_id: "fed".into(),
        execution_window_start: 0,
        execution_window_end: 100,
        ticks: vec![ScheduledTick {
            tick: 1,
            dag_node: "n1".into(),
            sync_window: 8,
        }],
    };
    assert_eq!(schedule.scheduling_hash(), schedule.scheduling_hash());

    let ev = EvernodeDeploymentManifest {
        deployment_manifest: "d".into(),
        anchor_coordination: "a".into(),
        settlement_reference: "s".into(),
        appliance_continuity: "c".into(),
        topology_anchor: "t".into(),
    };
    assert_eq!(ev.integration_hash(), ev.integration_hash());

    let sdk = SdkProjectManifest {
        world_id: "w".into(),
        asset_package_hash: "h".into(),
        topology_hash: "t".into(),
        replay_profile: "strict".into(),
        deployment_profile: "offline".into(),
    };
    assert_eq!(sdk.manifest_hash(), sdk.manifest_hash());

    let diverged = DiagnosticManifest {
        replay_root_a: "1".into(),
        replay_root_b: "2".into(),
        checkpoint_root_a: "3".into(),
        checkpoint_root_b: "3".into(),
        settlement_root: "s".into(),
        topology_root: "t".into(),
        continuity_summary: "diff".into(),
    };
    assert!(diverged.has_divergence());
}
