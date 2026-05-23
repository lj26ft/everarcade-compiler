use execution_core::wasm::engine::deterministic_engine_config_hash;

#[test]
fn engine_config_hash_stable() {
    assert_eq!(deterministic_engine_config_hash().len(), 64);
}
