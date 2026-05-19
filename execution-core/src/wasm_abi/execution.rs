use super::{
    abi::{AbiRequest, AbiResponse, EVERARCADE_ABI_VERSION},
    errors::WasmAbiError,
    fuel::{FuelConsumed, FuelLimit, FuelReport},
    layout,
    memory::{allocate_input_buffer, read_response, write_request},
    validation::validate_module_bytes,
};
use wasmtime::{Config, Engine, Instance, Linker, Module, Store};

fn deterministic_engine() -> Result<Engine, WasmAbiError> {
    let mut config = Config::new();
    config.wasm_simd(false);
    config.wasm_relaxed_simd(false);
    config.wasm_tail_call(false);
    config.consume_fuel(true);
    Engine::new(&config).map_err(|e| WasmAbiError::Runtime(e.to_string()))
}

pub fn execute_contract(
    wasm: &[u8],
    request: &AbiRequest,
    fuel_limit: FuelLimit,
) -> Result<(AbiResponse, FuelReport), WasmAbiError> {
    if request.context.abi_version != EVERARCADE_ABI_VERSION {
        return Err(WasmAbiError::AbiVersionMismatch {
            expected: EVERARCADE_ABI_VERSION,
            actual: request.context.abi_version,
        });
    }
    let engine = deterministic_engine()?;
    validate_module_bytes(&engine, wasm)?;
    let module =
        Module::from_binary(&engine, wasm).map_err(|e| WasmAbiError::Runtime(e.to_string()))?;
    let mut store = Store::new(&engine, ());
    store
        .set_fuel(fuel_limit.0)
        .map_err(|e| WasmAbiError::Runtime(e.to_string()))?;

    let linker = Linker::new(&engine);
    let instance = linker
        .instantiate(&mut store, &module)
        .map_err(|e| WasmAbiError::Runtime(e.to_string()))?;
    execute_instance(&mut store, &instance, request, fuel_limit)
}

fn execute_instance(
    store: &mut Store<()>,
    instance: &Instance,
    request: &AbiRequest,
    fuel_limit: FuelLimit,
) -> Result<(AbiResponse, FuelReport), WasmAbiError> {
    let memory = instance
        .get_memory(&mut *store, layout::MEMORY_EXPORT)
        .ok_or(WasmAbiError::MissingExport(layout::MEMORY_EXPORT))?;
    let alloc = instance
        .get_typed_func::<i32, i32>(&mut *store, layout::ALLOC_EXPORT)
        .map_err(|_| WasmAbiError::MissingExport(layout::ALLOC_EXPORT))?;
    let entry = instance
        .get_typed_func::<(i32, i32), i64>(&mut *store, layout::ENTRYPOINT)
        .map_err(|_| WasmAbiError::MissingExport(layout::ENTRYPOINT))?;

    let req_len = super::serialization::serialize(request)?.len();
    let req_ptr = allocate_input_buffer(&mut *store, &alloc, req_len)?;
    let written = write_request(&mut *store, &memory, req_ptr, request)?;
    let handle = entry
        .call(&mut *store, (req_ptr as i32, written as i32))
        .map_err(|e| WasmAbiError::Runtime(e.to_string()))? as u64;
    let (resp_ptr, resp_len) = layout::decode_return_handle(handle);
    if resp_len == 0 {
        return Err(WasmAbiError::InvalidResultHandle);
    }

    let response: AbiResponse = read_response(&mut *store, &memory, resp_ptr, resp_len)?;
    let consumed = fuel_limit
        .0
        .saturating_sub(store.get_fuel().unwrap_or(fuel_limit.0));
    let report = FuelReport {
        limit: fuel_limit,
        consumed: FuelConsumed(consumed),
        exhausted: consumed >= fuel_limit.0,
    };
    Ok((response, report))
}
