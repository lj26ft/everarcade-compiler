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
        ..Default::default()
    };
    let receipt = runtime.enforce_governance(&policy, 0, 0, 0, 0, 0, 0, 0, true, true);
    assert_eq!(receipt.status, ExecutionStatus::ReplayOverflow);
    let receipt2 = runtime.enforce_governance(&policy, 0, 0, 0, 0, 0, 0, 0, true, true);
    assert_eq!(receipt, receipt2);
}
