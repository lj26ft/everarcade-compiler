use crate::{VmInput, VmOutput};

use super::engine::deterministic_engine;
use super::instance::instantiate;
use super::limits::ExecutionLimits;
use super::memory::{deserialize_abi, read_memory, serialize_abi, write_memory};
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
            .context("wasm module missing exported memory")?;

        let alloc = instance
            .get_typed_func::<i32, i32>(&mut store, "alloc")
            .context("wasm module missing alloc export")?;

        let execute = instance
            .get_typed_func::<(i32, i32), i32>(&mut store, "execute")
            .context("wasm module missing execute export")?;

        let input_bytes = serialize_abi(&input)?;
        let input_len = i32::try_from(input_bytes.len()).context("input too large")?;
        let input_ptr = alloc.call(&mut store, input_len)?;

        write_memory(&mut store, &memory, input_ptr, &input_bytes)?;

        let output_ptr = execute.call(&mut store, (input_ptr, input_len))?;
        if output_ptr < 0 {
            anyhow::bail!("execute returned negative pointer")
        }

        let output_len = instance
            .get_typed_func::<(), i32>(&mut store, "output_len")
            .context("wasm module missing output_len export")?
            .call(&mut store, ())?;

        let output_bytes = read_memory(&mut store, &memory, output_ptr, output_len)?;
        let output: VmOutput = deserialize_abi(&output_bytes)?;

        Ok(output)
    }
}
