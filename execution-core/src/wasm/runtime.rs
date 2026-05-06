use crate::{VmInput, VmOutput};

use super::engine::deterministic_engine;
use super::instance::instantiate;
use super::limits::ExecutionLimits;
use super::memory::{read_len_prefixed, write_len_prefixed};
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

        let vm_alloc = instance
            .get_typed_func::<i32, i32>(&mut store, "vm_alloc")
            .context("wasm module missing vm_alloc export")?;

        let vm_run = instance
            .get_typed_func::<i32, i32>(&mut store, "vm_run")
            .context("wasm module missing vm_run export")?;

        let input_bytes = serde_json::to_vec(&input)?;
        let frame_size = i32::try_from(input_bytes.len() + 4).context("input too large")?;
        let input_ptr = vm_alloc.call(&mut store, frame_size)?;

        write_len_prefixed(&mut store, &memory, input_ptr as usize, &input_bytes)?;

        let output_ptr = vm_run.call(&mut store, input_ptr)?;
        if output_ptr < 0 {
            anyhow::bail!("vm_run returned negative pointer")
        }

        let output_bytes = read_len_prefixed(&mut store, &memory, output_ptr as usize)?;
        let output: VmOutput = serde_json::from_slice(&output_bytes)?;

        Ok(output)
    }
}
