use wasmtime::{Config, Engine};

use crate::hashing::sha256;

pub fn deterministic_engine() -> anyhow::Result<Engine> {
    Ok(Engine::new(&deterministic_engine_config())?)
}

pub fn deterministic_engine_config() -> Config {
    let mut config = Config::new();
    config.consume_fuel(true);
    config.wasm_simd(false);
    config.wasm_relaxed_simd(false);
    config.wasm_tail_call(false);
    config.wasm_multi_memory(false);
    config.wasm_bulk_memory(true);
    config
}

pub fn deterministic_engine_config_hash() -> String {
    let canonical = b"fuel=1;simd=0;relaxed_simd=0;tail_call=0;multi_memory=0;bulk_memory=1";
    hex::encode(sha256(canonical))
}
