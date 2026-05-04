use wasmtime::{AsContextMut, Caller, Memory};

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

pub fn _caller_placeholder(_caller: Caller<'_, ()>) {
    // host ABI hooks will be added here in step 2.
}
