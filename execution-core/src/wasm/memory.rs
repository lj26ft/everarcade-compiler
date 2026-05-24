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
    pub fn verify_export(e: &MemoryExportEnvelope) -> anyhow::Result<()> {
        let mut regions = e.layout.regions.clone();
        regions.sort_by_key(|r| r.offset);
        let mut last_end = 0u32;
        for r in regions {
            if r.length == 0 {
                anyhow::bail!("zero-length memory region")
            }
            if r.offset < last_end {
                anyhow::bail!("overlapping memory region")
            }
            let end = r
                .offset
                .checked_add(r.length)
                .ok_or_else(|| anyhow::anyhow!("memory region overflow"))?;
            if end as usize > e.bytes.len() {
                anyhow::bail!("memory layout exceeds export bytes")
            }
            last_end = end;
        }
        Ok(())
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
