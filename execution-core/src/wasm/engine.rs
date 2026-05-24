use serde::{Deserialize, Serialize};
use wasmtime::{Config, Engine, Module};

use crate::hashing::sha256;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicExecutionConfig {
    pub enable_fuel: bool,
    pub enable_bulk_memory: bool,
    pub enable_multi_memory: bool,
    pub enable_tail_calls: bool,
    pub enable_simd: bool,
    pub enable_relaxed_simd: bool,
    pub enable_threads: bool,
}

impl Default for DeterministicExecutionConfig {
    fn default() -> Self {
        Self {
            enable_fuel: true,
            enable_bulk_memory: true,
            enable_multi_memory: false,
            enable_tail_calls: false,
            enable_simd: false,
            enable_relaxed_simd: false,
            enable_threads: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionFuelPolicy {
    pub fuel_limit: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionEnvironment {
    pub config_hash: String,
    pub module_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleLoadReceipt {
    pub module_hash: String,
    pub config_hash: String,
    pub status: String,
}

pub struct DeterministicWasmEngine {
    pub config: DeterministicExecutionConfig,
    pub engine: Engine,
}

impl DeterministicWasmEngine {
    pub fn new(config: DeterministicExecutionConfig) -> anyhow::Result<Self> {
        let mut c = Config::new();
        c.consume_fuel(config.enable_fuel);
        c.wasm_bulk_memory(config.enable_bulk_memory);
        c.wasm_multi_memory(config.enable_multi_memory);
        c.wasm_tail_call(config.enable_tail_calls);
        c.wasm_simd(config.enable_simd);
        c.wasm_relaxed_simd(config.enable_relaxed_simd);
        Ok(Self {
            config,
            engine: Engine::new(&c)?,
        })
    }

    pub fn module_hash(&self, module: &[u8]) -> String {
        hex::encode(sha256(module))
    }

    pub fn config_hash(&self) -> String {
        let canonical = serde_json::to_vec(&self.config).unwrap_or_default();
        hex::encode(sha256(&canonical))
    }

    pub fn compile_module(&self, module: &[u8]) -> anyhow::Result<ModuleLoadReceipt> {
        Module::from_binary(&self.engine, module)?;
        Ok(ModuleLoadReceipt {
            module_hash: self.module_hash(module),
            config_hash: self.config_hash(),
            status: "module_loaded".to_string(),
        })
    }

    pub fn environment(&self, module: &[u8]) -> ExecutionEnvironment {
        ExecutionEnvironment {
            config_hash: self.config_hash(),
            module_hash: self.module_hash(module),
        }
    }
}

pub fn deterministic_engine() -> anyhow::Result<Engine> {
    Ok(DeterministicWasmEngine::new(DeterministicExecutionConfig::default())?.engine)
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
    let canonical =
        b"fuel=1;simd=0;relaxed_simd=0;tail_call=0;multi_memory=0;bulk_memory=1;threads=0";
    hex::encode(sha256(canonical))
}
