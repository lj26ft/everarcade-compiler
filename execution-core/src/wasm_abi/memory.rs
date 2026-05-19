use super::{errors::WasmAbiError, serialization};
use wasmtime::{AsContextMut, Memory};

pub fn allocate_input_buffer(
    mut store: impl AsContextMut<Data = ()>,
    alloc: &wasmtime::TypedFunc<i32, i32>,
    len: usize,
) -> Result<u32, WasmAbiError> {
    let ptr = alloc
        .call(
            &mut store,
            i32::try_from(len).map_err(|_| WasmAbiError::MemoryBounds)?,
        )
        .map_err(|e| WasmAbiError::Runtime(e.to_string()))?;
    u32::try_from(ptr).map_err(|_| WasmAbiError::MemoryBounds)
}

pub fn write_request<T: serde::Serialize>(
    mut store: impl AsContextMut<Data = ()>,
    memory: &Memory,
    ptr: u32,
    request: &T,
) -> Result<usize, WasmAbiError> {
    let bytes = serialization::serialize(request)?;
    memory
        .write(&mut store, ptr as usize, &bytes)
        .map_err(|_| WasmAbiError::MemoryBounds)?;
    Ok(bytes.len())
}

pub fn read_response<T: serde::de::DeserializeOwned>(
    mut store: impl AsContextMut<Data = ()>,
    memory: &Memory,
    ptr: u32,
    len: u32,
) -> Result<T, WasmAbiError> {
    let mut buf = vec![0u8; len as usize];
    memory
        .read(&mut store, ptr as usize, &mut buf)
        .map_err(|_| WasmAbiError::MemoryBounds)?;
    serialization::deserialize(&buf)
}
