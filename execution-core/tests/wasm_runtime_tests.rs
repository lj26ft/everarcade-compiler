use execution_core::wasm::{
    abi::{decode, encode, AbiExecutionRequest, AbiMutationSet, CanonicalAbiEnvelope},
    engine::{DeterministicExecutionConfig, DeterministicWasmEngine},
    memory::{
        CanonicalMemoryLayout, DeterministicMemoryBridge, MemoryExportEnvelope, MemoryRegion,
    },
    mutations::ExecutionMutationSet,
    state::StatefulExecutionRuntime,
};

#[test]
fn deterministic_module_loading_and_receipt() {
    let eng = DeterministicWasmEngine::new(DeterministicExecutionConfig::default()).unwrap();
    let wasm = wat::parse_str("(module (func (export \"everarcade_execute\") (param i32 i32) (result i64) i64.const 0) (func (export \"alloc\") (param i32) (result i32) local.get 0) (memory (export \"memory\") 1))").unwrap();
    let loaded = eng.compile_module(&wasm).unwrap();
    assert_eq!(eng.config_hash(), loaded.config_hash);
    assert_eq!(eng.module_hash(&wasm), loaded.module_hash);
}

#[test]
fn host_memory_bridge_canonical_roundtrip() {
    let req = AbiExecutionRequest {
        contract_id: "demo".into(),
        input: b"inc".to_vec(),
    };
    let envelope = CanonicalAbiEnvelope {
        version: 1,
        payload: encode(&req).unwrap(),
    };
    let bytes = encode(&envelope).unwrap();
    let layout = CanonicalMemoryLayout {
        regions: vec![MemoryRegion {
            offset: 0,
            length: bytes.len() as u32,
        }],
    };
    let exp = MemoryExportEnvelope {
        layout,
        bytes: bytes.clone(),
    };
    DeterministicMemoryBridge::verify_export(&exp).unwrap();
    let out: CanonicalAbiEnvelope = decode(&bytes).unwrap();
    let decoded: AbiExecutionRequest = decode(&out.payload).unwrap();
    assert_eq!(decoded.input, b"inc".to_vec());
}

#[test]
fn overlapping_memory_region_rejected() {
    let exp = MemoryExportEnvelope {
        layout: CanonicalMemoryLayout {
            regions: vec![
                MemoryRegion {
                    offset: 0,
                    length: 4,
                },
                MemoryRegion {
                    offset: 2,
                    length: 4,
                },
            ],
        },
        bytes: vec![0; 10],
    };
    assert!(DeterministicMemoryBridge::verify_export(&exp).is_err());
}

#[test]
fn duplicate_mutations_rejected() {
    let m = ExecutionMutationSet {
        entries: vec![("k".into(), vec![1]), ("k".into(), vec![2])],
    };
    assert!(StatefulExecutionRuntime::validate_mutations(&m).is_err());
}

#[test]
fn malformed_abi_deterministic_failure() {
    let bad = b"{\"output\":1}".to_vec();
    let err1 = decode::<AbiMutationSet>(&bad).unwrap_err().to_string();
    let err2 = decode::<AbiMutationSet>(&bad).unwrap_err().to_string();
    assert_eq!(err1, err2);
}
