use execution_core::wasm::types::WasmExecutionManifest;

#[test]
fn wasm_manifest_hash_is_stable() {
    let m = WasmExecutionManifest {
        module_hash: "m".into(),
        input_hash: "i".into(),
        host_abi_version: "1".into(),
        fuel_limit: 10,
        deterministic_engine_config_hash: "e".into(),
        initial_state_root: "s0".into(),
        final_state_root: "s1".into(),
        receipt_hash: "r".into(),
        state_diff_hash: "d".into(),
        journal_hash: "j".into(),
    };
    assert_eq!(m.canonical_hash().unwrap(), m.canonical_hash().unwrap());
}
