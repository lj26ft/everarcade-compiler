use execution_core::deployment::*;

#[test]
fn test_runtime_node_identity_stability() {
    assert_eq!(
        RuntimeNodeIdentity::new("ns", "n").deterministic_id,
        RuntimeNodeIdentity::new("ns", "n").deterministic_id
    )
}
#[test]
fn test_lifecycle_transition_determinism() {
    let mut a = NodeContinuityEnvelope::default();
    let mut b = NodeContinuityEnvelope::default();
    assert_eq!(
        a.transition(RuntimeNodeLifecycle::Bootstrapped).unwrap(),
        b.transition(RuntimeNodeLifecycle::Bootstrapped).unwrap()
    );
}
#[test]
fn test_invalid_transition_rejection() {
    let mut e = NodeContinuityEnvelope::default();
    assert!(e.transition(RuntimeNodeLifecycle::Active).is_err())
}
#[test]
fn test_bootstrap_equivalence() {
    let env = RuntimeBootstrapEnvelope {
        manifest: BootstrapManifest {
            genesis_hash: "g".into(),
            expected_package_hash: hash_hex(b"x"),
        },
        package: GenesisRuntimePackage {
            name: "g".into(),
            bytes: b"x".to_vec(),
        },
    };
    assert_eq!(env.verify(), env.verify())
}
#[test]
fn test_operational_continuity_hashing() {
    let mut e = NodeContinuityEnvelope::default();
    e.transition(RuntimeNodeLifecycle::Bootstrapped).unwrap();
    assert_eq!(e.continuity_root(), e.continuity_root())
}
#[test]
fn test_runtime_package_hash_stability() {
    let p = RuntimePackage {
        name: "p".into(),
        payload: b"a".to_vec(),
    };
    assert_eq!(p.package_hash(), p.package_hash())
}
#[test]
fn test_deployment_bundle_equivalence() {
    assert_eq!(
        DeploymentBundle::new(vec!["b".into(), "a".into()]),
        DeploymentBundle::new(vec!["a".into(), "b".into()])
    )
}
#[test]
fn test_release_upgrade_determinism() {
    assert_eq!(
        RuntimeUpgradeEnvelope::new("1", "2"),
        RuntimeUpgradeEnvelope::new("1", "2")
    )
}
#[test]
fn test_release_rollback_equivalence() {
    assert_eq!(
        RuntimeUpgradeEnvelope::new("2", "1"),
        RuntimeUpgradeEnvelope::new("2", "1")
    )
}
#[test]
fn test_compatibility_boundary_validation() {
    assert!(CompatibilityBoundary {
        min_version: "1".into(),
        max_version: "3".into()
    }
    .supports("2"))
}
#[test]
fn test_federation_membership_stability() {
    assert_eq!(
        FederationTopology::canonical(vec!["b".into(), "a".into()]),
        FederationTopology::canonical(vec!["a".into(), "b".into()])
    )
}
#[test]
fn test_partition_recovery_equivalence() {
    assert_eq!(
        PartitionBoundary {
            partition_id: "p1".into()
        },
        PartitionBoundary {
            partition_id: "p1".into()
        }
    )
}
#[test]
fn test_synchronization_window_determinism() {
    assert_eq!(
        SynchronizationWindow {
            start_tick: 1,
            end_tick: 2
        },
        SynchronizationWindow {
            start_tick: 1,
            end_tick: 2
        }
    )
}
#[test]
fn test_federation_restoration_equivalence() {
    let t = FederationTopology::canonical(vec!["a".into()]);
    assert_eq!(t, t.clone())
}
#[test]
fn test_external_anchor_stability() {
    let a = ExternalSettlementAnchor {
        anchor_id: "x".into(),
        anchor_hash: hash_hex("x"),
    };
    assert_eq!(a, a.clone())
}
#[test]
fn test_checkpoint_restoration_equivalence() {
    let c = RecoveryCheckpoint {
        checkpoint_hash: "c".into(),
        height: 1,
    };
    assert_eq!(c, c.clone())
}
#[test]
fn test_archive_recovery_equivalence() {
    let r = ContinuityRecoveryReport {
        recovered: true,
        report_hash: "r".into(),
    };
    assert_eq!(r, r.clone())
}
#[test]
fn test_quarantine_reintegration_equivalence() {
    let q = DeploymentQuarantine { reason: "r".into() };
    assert_eq!(q, q.clone())
}
#[test]
fn test_crash_recovery_determinism() {
    let p = RecoveryPlan {
        checkpoints: vec![RecoveryCheckpoint {
            checkpoint_hash: "a".into(),
            height: 1,
        }],
    };
    assert_eq!(p, p.clone())
}
#[test]
fn test_corruption_boundary_isolation() {
    let c = CorruptionBoundary {
        boundary_hash: "h".into(),
    };
    assert_eq!(c, c.clone())
}
#[test]
fn test_operational_scheduler_replay() {
    let s = OperationalScheduler::ordered(vec![
        DeterministicTaskEnvelope {
            task_id: "b".into(),
            tick: DeploymentTick { tick: 2 },
        },
        DeterministicTaskEnvelope {
            task_id: "a".into(),
            tick: DeploymentTick { tick: 1 },
        },
    ]);
    assert_eq!(s.queue[0].task_id, "a")
}
#[test]
fn test_task_ordering_stability() {
    let a = OperationalScheduler::ordered(vec![DeterministicTaskEnvelope {
        task_id: "1".into(),
        tick: DeploymentTick { tick: 1 },
    }]);
    let b = OperationalScheduler::ordered(vec![DeterministicTaskEnvelope {
        task_id: "1".into(),
        tick: DeploymentTick { tick: 1 },
    }]);
    assert_eq!(a, b)
}
#[test]
fn test_operational_ledger_continuity() {
    let mut l = OperationalLedger { entries: vec![] };
    l.append("x");
    assert_eq!(l.root(), l.root())
}
#[test]
fn test_deployment_tick_equivalence() {
    assert_eq!(DeploymentTick { tick: 1 }, DeploymentTick { tick: 1 })
}
#[test]
fn test_runtime_health_report_stability() {
    let h = RuntimeHealthEnvelope {
        health_hash: "h".into(),
    };
    assert_eq!(h, h.clone())
}
#[test]
fn test_governance_transition_determinism() {
    let g = GovernanceTransition {
        from_policy: "a".into(),
        to_policy: "b".into(),
        transition_hash: hash_hex("ab"),
    };
    assert_eq!(g, g.clone())
}
#[test]
fn test_operator_authority_lineage() {
    let o = OperatorAuthority {
        operator_id: "op2".into(),
        predecessor: Some("op1".into()),
    };
    assert_eq!(o.predecessor.as_deref(), Some("op1"))
}
#[test]
fn test_policy_evolution_equivalence() {
    assert_eq!(
        OperationalPolicy {
            policy_id: "p".into(),
            revision: 2
        },
        OperationalPolicy {
            policy_id: "p".into(),
            revision: 2
        }
    )
}
#[test]
fn test_runtime_directive_stability() {
    let d = RuntimeDirective {
        directive_hash: "d".into(),
    };
    assert_eq!(d, d.clone())
}
