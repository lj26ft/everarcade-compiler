use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofChunk {
    pub index: usize,
    pub total: usize,
    pub bytes: Vec<u8>,
}

pub fn chunk_proof(bytes: &[u8], chunk_size: usize) -> Vec<ProofChunk> {
    let size = chunk_size.max(1);
    let total = bytes.len().div_ceil(size);
    bytes.chunks(size)
        .enumerate()
        .map(|(index, c)| ProofChunk { index, total, bytes: c.to_vec() })
        .collect()
}

pub fn reconstruct_proof(chunks: &[ProofChunk]) -> Vec<u8> {
    let mut ordered = chunks.to_vec();
    ordered.sort_by_key(|c| c.index);
    ordered.into_iter().flat_map(|c| c.bytes).collect()
}
