use crate::{VmInput, VmOutput};

use super::engine::deterministic_engine;
use super::instance::instantiate;
use super::limits::ExecutionLimits;
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

        let _vm_run = instance.get_typed_func::<i32, i32>(&mut store, "vm_run")?;

        // Step 1 intentionally stops at deterministic engine + module instantiation.
        // Step 2 wires ABI memory encoding/decoding for vm_run.
        let _ = input;
        anyhow::bail!("vm_run ABI bridge not yet connected")
    }
}
