use execution_core::wasm::execution::ExecutionStatus;
use execution_core::world::runtime::{
    IncrementalWorldRuntime, RuntimeGovernancePolicy, WorldRuntimeTick,
};

fn setup_runtime() -> IncrementalWorldRuntime {
    let mut runtime = IncrementalWorldRuntime::default();
    runtime
        .advance(
            WorldRuntimeTick {
                tick: 1,
                workload_partitions: vec!["p0".into()],
            },
            1,
        )
        .unwrap();
    runtime
}

#[test]
fn test_mutation_budget_enforcement() {
    let runtime = setup_runtime();
    let policy = RuntimeGovernancePolicy {
        max_mutations_per_execution: 1,
        ..Default::default()
    };
    assert_eq!(
        runtime
            .enforce_governance(&policy, 2, 0, 0, 0, 0, 0, 0, true, true)
            .status,
        ExecutionStatus::ResourceLimitExceeded
    );
}
#[test]
fn test_event_budget_enforcement() {
    let runtime = setup_runtime();
    let policy = RuntimeGovernancePolicy {
        max_events_per_execution: 1,
        ..Default::default()
    };
    assert_eq!(
        runtime
            .enforce_governance(&policy, 0, 0, 2, 0, 0, 0, 0, true, true)
            .status,
        ExecutionStatus::EventOverflow
    );
}
#[test]
fn test_witness_budget_enforcement() {
    let runtime = setup_runtime();
    let policy = RuntimeGovernancePolicy {
        max_witness_chunk_size: 1,
        ..Default::default()
    };
    assert_eq!(
        runtime
            .enforce_governance(&policy, 0, 0, 0, 0, 2, 0, 0, true, true)
            .status,
        ExecutionStatus::WitnessOverflow
    );
}
#[test]
fn test_replay_budget_enforcement() {
    let runtime = setup_runtime();
    let policy = RuntimeGovernancePolicy {
        max_replay_window_depth: 0,
        ..Default::default()
    };
    assert_eq!(
        runtime
            .enforce_governance(&policy, 0, 0, 0, 0, 0, 0, 0, true, true)
            .status,
        ExecutionStatus::ReplayOverflow
    );
}
#[test]
fn test_snapshot_budget_enforcement() {
    let runtime = setup_runtime();
    let policy = RuntimeGovernancePolicy {
        max_snapshot_chain_depth: 0,
        ..Default::default()
    };
    assert_eq!(
        runtime
            .enforce_governance(&policy, 0, 0, 0, 0, 0, 0, 0, true, true)
            .status,
        ExecutionStatus::SnapshotOverflow
    );
}
#[test]
fn test_partition_merge_budget_enforcement() {
    let runtime = setup_runtime();
    let policy = RuntimeGovernancePolicy {
        max_partition_merge_inputs: 1,
        ..Default::default()
    };
    assert_eq!(
        runtime
            .enforce_governance(&policy, 0, 0, 0, 0, 0, 2, 0, true, true)
            .status,
        ExecutionStatus::ResourceLimitExceeded
    );
}
#[test]
fn test_validation_export_budget_enforcement() {
    let runtime = setup_runtime();
    let policy = RuntimeGovernancePolicy {
        max_validation_export_size: 1,
        ..Default::default()
    };
    assert_eq!(
        runtime
            .enforce_governance(&policy, 0, 0, 0, 0, 0, 0, 2, true, true)
            .status,
        ExecutionStatus::ResourceLimitExceeded
    );
}
#[test]
fn test_capability_violation_enforcement() {
    let runtime = setup_runtime();
    assert_eq!(
        runtime
            .enforce_governance(
                &RuntimeGovernancePolicy::default(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                false,
                true
            )
            .status,
        ExecutionStatus::CapabilityViolation
    );
}
#[test]
fn test_isolation_violation_enforcement() {
    let runtime = setup_runtime();
    assert_eq!(
        runtime
            .enforce_governance(
                &RuntimeGovernancePolicy::default(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
                false
            )
            .status,
        ExecutionStatus::IsolationViolation
    );
}
#[test]
fn test_governance_replay_equivalence() {
    let runtime = setup_runtime();
    let policy = RuntimeGovernancePolicy {
        max_events_per_execution: 1,
        ..Default::default()
    };
    let a = runtime.enforce_governance(&policy, 0, 0, 2, 0, 0, 0, 0, true, true);
    let b = runtime.enforce_governance(&policy, 0, 0, 2, 0, 0, 0, 0, true, true);
    assert_eq!(a, b);
}
#[test]
fn test_quarantine_replay_equivalence() {
    let runtime = setup_runtime();
    let a = runtime.enforce_governance(
        &RuntimeGovernancePolicy::default(),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        false,
        true,
    );
    let b = runtime.enforce_governance(
        &RuntimeGovernancePolicy::default(),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        false,
        true,
    );
    assert!(a.quarantined && b.quarantined);
    assert_eq!(a.rejection_root, b.rejection_root);
}
#[test]
fn test_security_root_stability_under_rejection() {
    let runtime = setup_runtime();
    let r = runtime.enforce_governance(
        &RuntimeGovernancePolicy::default(),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        false,
        true,
    );
    assert!(!r.rejection_root.is_empty());
}
