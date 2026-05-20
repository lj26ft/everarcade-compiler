use execution_core::wasm_abi::{
    abi::{AbiExecutionContext, AbiRequest, EVERARCADE_ABI_VERSION},
    layout::{decode_return_handle, encode_return_handle},
    serialization,
};

#[test]
fn test_abi_request_serialization_is_deterministic() {
    let req = AbiRequest {
        context: AbiExecutionContext {
            abi_version: EVERARCADE_ABI_VERSION,
            contract_id: "c1".into(),
            contract_version: "1.0.0".into(),
            previous_state_root: [1; 32],
            continuity_hash: [2; 32],
        },
        input: vec![1, 2, 3],
        state_reads: vec![],
    };
    let a = serialization::serialize(&req).unwrap();
    let b = serialization::serialize(&req).unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_guest_response_decoding() {
    let handle = encode_return_handle(42, 128);
    let (ptr, len) = decode_return_handle(handle);
    assert_eq!(ptr, 42);
    assert_eq!(len, 128);
}

#[test]
fn test_abi_version_mismatch_rejected() {
    let req = AbiRequest {
        context: AbiExecutionContext {
            abi_version: EVERARCADE_ABI_VERSION + 1,
            contract_id: "c1".into(),
            contract_version: "1.0.0".into(),
            previous_state_root: [1; 32],
            continuity_hash: [2; 32],
        },
        input: vec![],
        state_reads: vec![],
    };

    let wasm = wat::parse_str(
        r#"(module
      (memory (export "memory") 1)
      (func (export "alloc") (param i32) (result i32) i32.const 0)
      (func (export "everarcade_execute") (param i32 i32) (result i64) i64.const 0)
    )"#,
    )
    .unwrap();

    let err = execution_core::wasm_abi::execute_contract(
        &wasm,
        &req,
        execution_core::wasm_abi::FuelLimit(10),
    )
    .unwrap_err();
    assert!(format!("{err}").contains("ABI version mismatch"));
}
