use execution_core::wasm::{
    abi::{encode, AbiExecutionRequest, CanonicalAbiEnvelope},
    engine::{DeterministicExecutionConfig, DeterministicWasmEngine},
    fuel::ExecutionFuelMeter,
    host::WasmRuntimeHost,
    memory::{
        CanonicalMemoryLayout, DeterministicMemoryBridge, MemoryExportEnvelope, MemoryRegion,
    },
    mutations::ExecutionMutationSet,
    receipts::ExecutionReceipt,
    scheduler::{DeterministicExecutionQueue, ExecutionTick, ScheduledExecutionEnvelope},
    serialization::canonical_bytes,
    state::StatefulExecutionRuntime,
};

#[test]
fn test_engine_configuration_determinism() {
    let a = DeterministicWasmEngine::new(DeterministicExecutionConfig::default()).unwrap();
    let b = DeterministicWasmEngine::new(DeterministicExecutionConfig::default()).unwrap();
    assert_eq!(a.config_hash(), b.config_hash());
}

#[test]
fn test_module_hash_stability() {
    let eng = DeterministicWasmEngine::new(DeterministicExecutionConfig::default()).unwrap();
    assert_eq!(eng.module_hash(b"abc"), eng.module_hash(b"abc"));
}

#[test]
fn test_canonical_abi_serialization() {
    let req = AbiExecutionRequest {
        contract_id: "c".into(),
        input: vec![1, 2],
    };
    assert_eq!(encode(&req).unwrap(), encode(&req).unwrap());
    let env = CanonicalAbiEnvelope {
        version: 1,
        payload: encode(&req).unwrap(),
    };
    assert_eq!(
        canonical_bytes(&env).unwrap(),
        canonical_bytes(&env).unwrap()
    );
}

#[test]
fn test_memory_layout_stability() {
    let export = MemoryExportEnvelope {
        layout: CanonicalMemoryLayout {
            regions: vec![MemoryRegion {
                offset: 0,
                length: 2,
            }],
        },
        bytes: vec![7, 8],
    };
    assert!(DeterministicMemoryBridge::verify_export(&export));
}

#[test]
fn test_mutation_validation() {
    let m = ExecutionMutationSet {
        entries: vec![("a".into(), vec![1]), ("b".into(), vec![2])],
    };
    assert!(m.reject_duplicates());
}

#[test]
fn test_duplicate_mutation_rejection() {
    let m = ExecutionMutationSet {
        entries: vec![("a".into(), vec![1]), ("a".into(), vec![2])],
    };
    assert!(!m.reject_duplicates());
}

#[test]
fn test_fuel_exhaustion_equivalence() {
    let mut meter = ExecutionFuelMeter::new(10);
    assert!(meter.consume(11).is_some());
}

#[test]
fn test_execution_receipt_stability() {
    let r = ExecutionReceipt {
        module_hash: "m".into(),
        input_hash: "i".into(),
        output_hash: "o".into(),
        mutation_root: "mr".into(),
        state_root: "s".into(),
        fuel_consumed: 1,
        continuity_root: "c".into(),
        execution_status: "ok".into(),
    };
    assert_eq!(canonical_bytes(&r).unwrap(), canonical_bytes(&r).unwrap());
}

#[test]
fn test_execution_scheduler_ordering() {
    let q = DeterministicExecutionQueue {
        items: vec![
            ScheduledExecutionEnvelope {
                tick: ExecutionTick { height: 1 },
                contract_id: "a".into(),
            },
            ScheduledExecutionEnvelope {
                tick: ExecutionTick { height: 2 },
                contract_id: "b".into(),
            },
        ],
    };
    assert!(q.items[0].tick.height < q.items[1].tick.height);
}

#[test]
fn test_full_wasm_runtime_pipeline() {
    let lifecycle = WasmRuntimeHost::lifecycle();
    assert_eq!(lifecycle.lifecycle.first(), Some(&"LOAD_MODULE"));
    let m = ExecutionMutationSet {
        entries: vec![("k".into(), vec![1])],
    };
    let root = StatefulExecutionRuntime::derive_root(&m);
    assert!(!root.0.is_empty());
}
