use super::errors::WasmAbiError;
use wasmtime::{Engine, Module};

pub fn validate_module_bytes(engine: &Engine, wasm: &[u8]) -> Result<(), WasmAbiError> {
    let module =
        Module::from_binary(engine, wasm).map_err(|e| WasmAbiError::Runtime(e.to_string()))?;
    validate_module(&module)
}

pub fn validate_module(module: &Module) -> Result<(), WasmAbiError> {
    if module.get_export("everarcade_execute").is_none() {
        return Err(WasmAbiError::MissingExport("everarcade_execute"));
    }
    if module.get_export("memory").is_none() {
        return Err(WasmAbiError::MissingExport("memory"));
    }
    Ok(())
}
