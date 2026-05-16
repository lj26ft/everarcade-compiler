use crate::{VmInput, VmOutput};

use super::abi::{decode, decode_handle, encode};
use super::engine::deterministic_engine;
use super::errors::WasmRuntimeError;
use super::instance::instantiate;
use super::limits::ExecutionLimits;
use super::memory::{read_memory, write_memory};
use anyhow::Context;
use wasmtime::{Linker, Module, Store};

pub struct WasmEngine {
    engine: wasmtime::Engine,
    limits: ExecutionLimits,
}

impl WasmEngine {
    pub fn new(limits: ExecutionLimits) -> anyhow::Result<Self> {
        Ok(Self {
            engine: deterministic_engine()?,
            limits,
        })
    }

    pub fn execute(&self, wasm: &[u8], input: VmInput) -> anyhow::Result<VmOutput> {
        let module = Module::from_binary(&self.engine, wasm)?;
        let mut store = Store::new(&self.engine, ());
        store.set_fuel(self.limits.fuel)?;

        let linker = Linker::new(&self.engine);
        let instance = instantiate(&mut store, &linker, &module)?;

        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or(WasmRuntimeError::MissingMemory)?;
        let alloc = instance
            .get_typed_func::<i32, i32>(&mut store, "alloc")
            .map_err(|_| WasmRuntimeError::MissingAlloc)?;
        let execute = instance
            .get_typed_func::<(i32, i32), i64>(&mut store, "everarcade_execute")
            .map_err(|_| WasmRuntimeError::MissingEntrypoint)?;

        let input_bytes =
            encode(&input).map_err(|e| WasmRuntimeError::AbiSerialization(e.to_string()))?;
        let input_len = i32::try_from(input_bytes.len()).context("input too large")?;
        let input_ptr = alloc.call(&mut store, input_len)?;
        write_memory(&mut store, &memory, input_ptr, &input_bytes)?;

        let result = execute.call(&mut store, (input_ptr, input_len))? as u64;
        let (output_ptr, output_len) = decode_handle(result);
        if output_len == 0 {
            return Err(WasmRuntimeError::InvalidResultHandle.into());
        }

        let output_bytes = read_memory(&mut store, &memory, output_ptr as i32, output_len as i32)?;
        decode(&output_bytes)
            .map_err(|e| WasmRuntimeError::AbiDeserialization(e.to_string()).into())
    }
}
