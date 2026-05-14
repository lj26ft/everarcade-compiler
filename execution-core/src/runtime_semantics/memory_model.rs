#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryModel {
    bytes: Vec<u8>,
}

impl MemoryModel {
    pub fn with_size(size: usize) -> Self {
        Self {
            bytes: vec![0; size],
        }
    }

    pub fn read(&self, offset: usize, len: usize) -> Option<Vec<u8>> {
        self.bytes.get(offset..offset + len).map(|v| v.to_vec())
    }

    pub fn write(&mut self, offset: usize, data: &[u8]) -> bool {
        match self.bytes.get_mut(offset..offset + data.len()) {
            Some(slice) => {
                slice.copy_from_slice(data);
                true
            }
            None => false,
        }
    }

    pub fn digest(&self) -> u64 {
        self.bytes.iter().fold(0u64, |acc, b| {
            acc.wrapping_mul(16777619).wrapping_add(*b as u64)
        })
    }
}
