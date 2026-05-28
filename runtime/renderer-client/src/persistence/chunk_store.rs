use crate::transport_runtime::wire::ReplayChunkWireMessage;
use std::collections::BTreeMap;
#[derive(Debug, Clone, Default)]
pub struct LiveReplayChunkStore {
    pub chunks: BTreeMap<u64, ReplayChunkWireMessage>,
    pub continuity_root: String,
}
impl LiveReplayChunkStore {
    pub fn with_root(root: impl Into<String>) -> Self {
        Self {
            continuity_root: root.into(),
            ..Self::default()
        }
    }
    pub fn append(&mut self, chunk: ReplayChunkWireMessage) -> Result<(), String> {
        chunk.validate()?;
        if chunk.continuity_root != self.continuity_root {
            return Err("corrupted_replay_history_rejected".into());
        }
        if self.chunks.contains_key(&chunk.sequence) {
            return Err("duplicate_replay_chunk".into());
        }
        let expected = self.chunks.len() as u64;
        if chunk.sequence != expected {
            return Err("out_of_order_replay_chunk".into());
        }
        self.chunks.insert(chunk.sequence, chunk);
        Ok(())
    }
    pub fn tip(&self) -> u64 {
        self.chunks.len() as u64
    }
}
