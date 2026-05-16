use crate::{hashing::sha256, VmInput, VmOutput};

use super::abi::{decode, decode_handle, encode};
use super::engine::deterministic_engine;
use super::errors::WasmRuntimeError;
use super::instance::instantiate;
use super::limits::ExecutionLimits;
use super::memory::{read_memory, write_memory};
use super::receipt::{compute_diff_hash, events_hash, execution_id, state_root, WasmExecutionReceipt};
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
        Ok(self.execute_with_receipt(wasm, input)?.0)
    }

    pub fn execute_with_receipt(
        &self,
        wasm: &[u8],
        input: VmInput,
    ) -> anyhow::Result<(VmOutput, WasmExecutionReceipt)> {
        let module = Module::from_binary(&self.engine, wasm)?;
        let mut store = Store::new(&self.engine, ());
        store.set_fuel(self.limits.fuel)?;
        let pre_state_root = state_root(&input.state);
        let input_bytes =
            encode(&input).map_err(|e| WasmRuntimeError::AbiSerialization(e.to_string()))?;
        let input_hash = sha256(&input_bytes);

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

        let input_len = i32::try_from(input_bytes.len()).context("input too large")?;
        let input_ptr = alloc.call(&mut store, input_len)?;
        write_memory(&mut store, &memory, input_ptr, &input_bytes)?;

        let result = execute.call(&mut store, (input_ptr, input_len))? as u64;
        let (output_ptr, output_len) = decode_handle(result);
        if output_len == 0 {
            return Err(WasmRuntimeError::InvalidResultHandle.into());
        }

        let output_bytes = read_memory(&mut store, &memory, output_ptr as i32, output_len as i32)?;
        let output: VmOutput = decode(&output_bytes)
            .map_err(|e| WasmRuntimeError::AbiDeserialization(e.to_string()))?;

        let post_state_root = state_root(&output.updated_state);
        let wasm_hash = sha256(wasm);
        let fuel_used = self
            .limits
            .fuel
            .saturating_sub(store.get_fuel().unwrap_or(self.limits.fuel));
        let receipt = WasmExecutionReceipt {
            execution_id: execution_id(input_hash, wasm_hash, post_state_root),
            pre_state_root,
            post_state_root,
            input_hash,
            output_hash: sha256(&output_bytes),
            diff_hash: compute_diff_hash(&output.receipt.state_changes),
            events_hash: events_hash(&output_bytes),
            fuel_used,
            wasm_hash,
        };
        Ok((output, receipt))
    }
}
