use super::chunk::hash_bytes;

#[derive(Debug, Clone, Default)]
pub struct ReplayCompressionRuntime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCompressionChunk {
    pub sequence: u64,
    pub compressed_payload: Vec<u8>,
    pub original_hash: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayCompressionManifest {
    pub chunk_hashes: Vec<String>,
}

impl ReplayCompressionRuntime {
    pub fn compress(sequence: u64, payload: &[u8]) -> ReplayCompressionChunk {
        let compressed_payload = payload.iter().rev().copied().collect::<Vec<_>>();
        ReplayCompressionChunk { sequence, compressed_payload, original_hash: hash_bytes(payload) }
    }

    pub fn decompress(chunk: &ReplayCompressionChunk) -> Result<Vec<u8>, String> {
        let restored = chunk.compressed_payload.iter().rev().copied().collect::<Vec<_>>();
        if hash_bytes(&restored) != chunk.original_hash {
            return Err("compression_equivalence_mismatch".to_string());
        }
        Ok(restored)
    }
}
