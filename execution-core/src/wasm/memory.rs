use anyhow::Context;
use serde::{de::DeserializeOwned, Serialize};
use wasmtime::{AsContextMut, Memory};

fn validate_bounds(
    mut caller: impl AsContextMut<Data = ()>,
    memory: &Memory,
    ptr: i32,
    len: usize,
) -> anyhow::Result<usize> {
    if ptr < 0 {
        anyhow::bail!("memory error: negative pointer")
    }
    let start = ptr as usize;
    let mem_len = memory.data_size(&mut caller);
    let end = start
        .checked_add(len)
        .ok_or_else(|| anyhow::anyhow!("memory error: overflow"))?;
    if end > mem_len {
        anyhow::bail!("memory error: out of bounds")
    }
    Ok(start)
}

pub fn write_memory(
    mut caller: impl AsContextMut<Data = ()>,
    memory: &Memory,
    ptr: i32,
    bytes: &[u8],
) -> anyhow::Result<()> {
    let start = validate_bounds(&mut caller, memory, ptr, bytes.len())?;
    memory
        .write(&mut caller, start, bytes)
        .context("failed writing guest memory")?;
    Ok(())
}

pub fn read_memory(
    mut caller: impl AsContextMut<Data = ()>,
    memory: &Memory,
    ptr: i32,
    len: i32,
) -> anyhow::Result<Vec<u8>> {
    if len < 0 {
        anyhow::bail!("memory error: negative length")
    }
    let usize_len = len as usize;
    let start = validate_bounds(&mut caller, memory, ptr, usize_len)?;
    let mut out = vec![0_u8; usize_len];
    memory
        .read(&mut caller, start, &mut out)
        .context("failed reading guest memory")?;
    Ok(out)
}

pub fn serialize_abi<T: Serialize>(value: &T) -> anyhow::Result<Vec<u8>> {
    bincode::serialize(value).context("failed ABI serialization")
}

pub fn deserialize_abi<T: DeserializeOwned>(bytes: &[u8]) -> anyhow::Result<T> {
    bincode::deserialize(bytes).context("failed ABI deserialization")
}
