#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionReplayCompressionManifest {
    pub algorithm: String,
    pub chunk_count: u64,
    pub continuity_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionReplayCompressionChunk {
    pub sequence: u64,
    pub payload_hash: String,
}
