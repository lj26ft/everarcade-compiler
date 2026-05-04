use wasmtime::{Config, Engine};

pub fn deterministic_engine() -> anyhow::Result<Engine> {
    let mut config = Config::new();
    config.wasm_multi_memory(false);
    config.wasm_threads(false);
    config.wasm_simd(false);
    config.wasm_reference_types(false);
    config.wasm_relaxed_simd(false);
    config.wasm_tail_call(false);
    config.wasm_bulk_memory(true);
    config.consume_fuel(true);

    Ok(Engine::new(&config)?)
}
