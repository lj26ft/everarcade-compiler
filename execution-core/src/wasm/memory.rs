use anyhow::Context;
use serde::{de::DeserializeOwned, Serialize};
use wasmtime::{AsContextMut, Memory};

pub fn write_memory(
    mut caller: impl AsContextMut<Data = ()>,
    memory: &Memory,
    ptr: i32,
    bytes: &[u8],
) -> anyhow::Result<()> {
    if ptr < 0 {
        anyhow::bail!("negative memory pointer")
    }

    memory
        .write(&mut caller, ptr as usize, bytes)
        .context("failed writing guest memory")?;

    Ok(())
}

pub fn read_memory(
    mut caller: impl AsContextMut<Data = ()>,
    memory: &Memory,
    ptr: i32,
    len: i32,
) -> anyhow::Result<Vec<u8>> {
    if ptr < 0 || len < 0 {
        anyhow::bail!("negative memory pointer or length")
    }

    let mut out = vec![0_u8; len as usize];
    memory
        .read(&mut caller, ptr as usize, &mut out)
        .context("failed reading guest memory")?;

    Ok(out)
}

pub fn serialize_abi<T: Serialize>(value: &T) -> anyhow::Result<Vec<u8>> {
    bincode::serialize(value).context("failed ABI serialization")
}

pub fn deserialize_abi<T: DeserializeOwned>(bytes: &[u8]) -> anyhow::Result<T> {
    bincode::deserialize(bytes).context("failed ABI deserialization")
}
