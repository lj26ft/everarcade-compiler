use execution_core::runtime_semantics::{
    allocator::DeterministicAllocator,
    compatibility::CompatibilityStamp,
    deterministic_runtime::execute_canonical,
    fuel::FuelMeter,
    host_abi::HostFrame,
    isolation::{isolated, IsolationBoundary},
    memory_model::MemoryModel,
    transition::RuntimeTransition,
    verifier_runtime::replay_for_verifier,
};

#[test]
fn test_memory_determinism() {
    let mut a = MemoryModel::with_size(8);
    let mut b = MemoryModel::with_size(8);
    assert!(a.write(0, &[1, 2, 3]));
    assert!(b.write(0, &[1, 2, 3]));
    assert_eq!(a.digest(), b.digest());
}

#[test]
fn test_allocator_determinism() {
    let mut a = DeterministicAllocator::new();
    let mut b = DeterministicAllocator::new();
    assert_eq!(a.alloc(4, 4), b.alloc(4, 4));
    assert_eq!(a.alloc(3, 2), b.alloc(3, 2));
}

#[test]
fn test_host_abi_stability() {
    let frame = HostFrame::new(vec![7, 8, 9]);
    let encoded = frame.encode();
    assert_eq!(HostFrame::decode(&encoded), Some(frame));
}

#[test]
fn test_fuel_accounting_determinism() {
    let mut a = FuelMeter::new(100);
    let mut b = FuelMeter::new(100);
    assert_eq!(a.charge(25), b.charge(25));
    assert_eq!(a.remaining, b.remaining);
}

#[test]
fn test_contract_isolation() {
    let a = IsolationBoundary {
        contract_id: 1,
        memory_namespace: 10,
    };
    let b = IsolationBoundary {
        contract_id: 2,
        memory_namespace: 20,
    };
    assert!(isolated(&a, &b));
}

#[test]
fn test_runtime_transition_stability() {
    let a = RuntimeTransition::apply(1, 2, 3, 4);
    let b = RuntimeTransition::apply(1, 2, 3, 4);
    assert_eq!(a, b);
}

#[test]
fn test_cross_verifier_runtime_replay() {
    let input = HostFrame::new(vec![1, 2, 3]).encode();
    assert_eq!(
        execute_canonical(&input, 100),
        replay_for_verifier(&input, 100)
    );
}

#[test]
fn test_epoch_runtime_compatibility() {
    let stamp = CompatibilityStamp {
        epoch: 1,
        runtime_version: 1,
        trace_version: 1,
        proof_version: 1,
    };
    assert!(stamp.is_compatible_with(stamp));
}
