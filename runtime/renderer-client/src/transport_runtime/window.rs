use super::chunk::ReplayChunk;

#[derive(Debug, Clone, Default)]
pub struct ReplayWindow {
    pub start_sequence: u64,
    pub end_sequence: u64,
    pub chunks: Vec<ReplayChunk>,
}
