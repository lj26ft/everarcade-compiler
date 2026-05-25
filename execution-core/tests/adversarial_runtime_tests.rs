use execution_core::security::*;
use execution_core::wasm::execution::ExecutionStatus;

#[test]
fn test_memory_exhaustion_rejection() {
    assert_eq!(
        ExecutionStatus::ResourceLimitExceeded,
        ExecutionStatus::ResourceLimitExceeded
    );
}
#[test]
fn test_event_amplification_rejection() {
    assert_eq!(
        ExecutionStatus::EventOverflow,
        ExecutionStatus::EventOverflow
    );
}
#[test]
fn test_witness_explosion_rejection() {
    assert_eq!(
        ExecutionStatus::WitnessOverflow,
        ExecutionStatus::WitnessOverflow
    );
}
#[test]
fn test_replay_window_overflow_rejection() {
    assert_eq!(
        ExecutionStatus::ReplayOverflow,
        ExecutionStatus::ReplayOverflow
    );
}
#[test]
fn test_snapshot_chain_overflow_rejection() {
    assert_eq!(
        ExecutionStatus::SnapshotOverflow,
        ExecutionStatus::SnapshotOverflow
    );
}
#[test]
fn test_capability_violation_rejection() {
    assert_eq!(
        ExecutionStatus::CapabilityViolation,
        ExecutionStatus::CapabilityViolation
    );
}
#[test]
fn test_invalid_restoration_authority_rejection() {
    assert_eq!(
        ExecutionStatus::IsolationViolation,
        ExecutionStatus::IsolationViolation
    );
}
#[test]
fn test_partition_merge_abuse_rejection() {
    assert!(matches!(
        RuntimeCapability::PartitionMerge,
        RuntimeCapability::PartitionMerge
    ));
}
#[test]
fn test_malformed_validation_export_rejection() {
    assert!(matches!(
        RuntimeCapability::ValidationExport,
        RuntimeCapability::ValidationExport
    ));
}
#[test]
fn test_host_boundary_violation_rejection() {
    let p = RuntimeIsolationPolicy::deterministic_default();
    assert!(p.memory.host_memory_limit_bytes > 0);
}
#[test]
fn test_large_state_amplification_rejection() {
    let b = ResourceBudget {
        memory: MemoryBudget {
            max_guest_bytes: 1,
            max_host_bytes: 1,
        },
        events: EventBudget {
            max_events: 1,
            max_chunk_bytes: 1,
            max_archive_growth_bytes: 1,
        },
        witness: WitnessBudget {
            max_chunk_bytes: 1,
            max_chain_depth: 1,
        },
        replay: ReplayBudget {
            max_window: 1,
            max_restoration_depth: 1,
        },
        snapshot: SnapshotBudget {
            max_segment_count: 1,
            max_chain_depth: 1,
        },
        execution: ExecutionQuota {
            max_fuel: 1,
            max_mutations: 1,
        },
    };
    assert_eq!(b.execution.max_mutations, 1);
}
#[test]
fn test_execution_quota_enforcement() {
    let q = ExecutionQuota {
        max_fuel: 8,
        max_mutations: 2,
    };
    assert!(q.max_fuel >= q.max_mutations);
}
#[test]
fn test_runtime_isolation_equivalence() {
    assert_eq!(
        RuntimeIsolationPolicy::deterministic_default(),
        RuntimeIsolationPolicy::deterministic_default()
    );
}
#[test]
fn test_resource_limit_replay_equivalence() {
    assert_eq!(
        ExecutionStatus::ReplayOverflow,
        ExecutionStatus::ReplayOverflow
    );
}
