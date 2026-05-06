use anyhow::Context;
use wasmtime::{AsContextMut, Memory};

pub fn write_bytes(
    mut caller: impl AsContextMut<Data = ()>,
    memory: &Memory,
    offset: usize,
    data: &[u8],
) -> anyhow::Result<()> {
    memory.write(&mut caller, offset, data)?;
    Ok(())
}

pub fn read_bytes(
    mut caller: impl AsContextMut<Data = ()>,
    memory: &Memory,
    offset: usize,
    len: usize,
) -> anyhow::Result<Vec<u8>> {
    let mut out = vec![0_u8; len];
    memory.read(&mut caller, offset, &mut out)?;
    Ok(out)
}

pub fn write_len_prefixed(
    mut caller: impl AsContextMut<Data = ()>,
    memory: &Memory,
    offset: usize,
    payload: &[u8],
) -> anyhow::Result<()> {
    let len = u32::try_from(payload.len()).context("payload too large")?;
    write_bytes(&mut caller, memory, offset, &len.to_le_bytes())?;
    write_bytes(&mut caller, memory, offset + 4, payload)?;
    Ok(())
}

pub fn read_len_prefixed(
    mut caller: impl AsContextMut<Data = ()>,
    memory: &Memory,
    offset: usize,
) -> anyhow::Result<Vec<u8>> {
    let len_bytes = read_bytes(&mut caller, memory, offset, 4)?;
    let len = u32::from_le_bytes(len_bytes.try_into().expect("length prefix")) as usize;
    read_bytes(&mut caller, memory, offset + 4, len)
}
