use execution_core::wasm::execution::ExecutionStatus;
use execution_core::world::runtime::{
    IncrementalWorldRuntime, RuntimeGovernancePolicy, WorldRuntimeTick,
};

#[test]
fn governance_abuse_scale_rejection_is_deterministic() {
    let mut runtime = IncrementalWorldRuntime::default();
    for t in 0..100_001u64 {
        runtime
            .advance(
                WorldRuntimeTick {
                    tick: t,
                    workload_partitions: vec!["p".into()],
                },
                1,
            )
            .unwrap();
    }
    let policy = RuntimeGovernancePolicy {
        max_replay_window_depth: 100_000,
        max_mutations_per_execution: u64::MAX,
        max_mutation_bytes: u64::MAX,
        max_events_per_execution: u64::MAX,
        max_event_chunk_size: u64::MAX,
        max_event_window_size: u64::MAX,
        max_witness_chunk_size: u64::MAX,
        max_witness_chain_depth: u64::MAX,
        max_snapshot_chain_depth: u64::MAX,
        max_partition_merge_inputs: u64::MAX,
        max_validation_export_size: u64::MAX,
        ..Default::default()
    };
    let receipt = runtime.enforce_governance(&policy, 0, 0, 0, 0, 0, 0, 0, true, true);
    assert_eq!(receipt.status, ExecutionStatus::ReplayOverflow);
    let receipt2 = runtime.enforce_governance(&policy, 0, 0, 0, 0, 0, 0, 0, true, true);
    assert_eq!(receipt, receipt2);
}
