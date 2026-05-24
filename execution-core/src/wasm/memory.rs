use serde::{Deserialize, Serialize};
use wasmtime::{Memory, Store};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRegion {
    pub offset: u32,
    pub length: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalMemoryLayout {
    pub regions: Vec<MemoryRegion>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryExportEnvelope {
    pub layout: CanonicalMemoryLayout,
    pub bytes: Vec<u8>,
}

pub struct DeterministicMemoryBridge;
impl DeterministicMemoryBridge {
    pub fn verify_export(e: &MemoryExportEnvelope) -> bool {
        e.layout
            .regions
            .iter()
            .map(|r| r.length as usize)
            .sum::<usize>()
            <= e.bytes.len()
    }
}

pub fn write_memory(
    store: &mut Store<()>,
    memory: &Memory,
    ptr: i32,
    bytes: &[u8],
) -> anyhow::Result<()> {
    memory.write(store, ptr as usize, bytes)?;
    Ok(())
}
pub fn read_memory(
    store: &mut Store<()>,
    memory: &Memory,
    ptr: i32,
    len: i32,
) -> anyhow::Result<Vec<u8>> {
    let mut out = vec![0u8; len as usize];
    memory.read(store, ptr as usize, &mut out)?;
    Ok(out)
}
